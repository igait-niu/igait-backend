use std::sync::Arc;

use async_openai::{
    config::OpenAIConfig, types::{
        AssistantObject, CreateAssistantToolFileSearchResources, CreateAssistantToolResources, CreateMessageRequestArgs, CreateRunRequest, CreateThreadRequest, MessageContent, MessageContentTextAnnotations, MessageRole, RunStatus, ThreadObject
    }, Client
};
use anyhow::{Result, Context, bail};
use tokio::time::{Duration, sleep};
use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::Response};
use serde::{Serialize, Deserialize};

use crate::helper::lib::AppState;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum AssistantUpdate {
    Message { content: String },
    Error { content: String },
    Waiting { content: String }
}

async fn send_response (
    client: &Client<OpenAIConfig>,
    thread: &ThreadObject,
    assistant: &AssistantObject,
    message: &str,
    socket: &mut WebSocket
) -> Result<()> {
    // Create and add the message
    let create_message_request = CreateMessageRequestArgs::default()
        .role(MessageRole::User)
        .content(message)
        .build()?;
    let _message_obj = client
        .threads()
        .messages(&thread.id)
        .create(create_message_request)
        .await?;

    // Create the run
    let create_run_request = CreateRunRequest {
        assistant_id: assistant.id.clone(),
        ..Default::default()
    };
    let mut run = client
        .threads()
        .runs(&thread.id)
        .create(create_run_request)
        .await?;

    // Poll the run object until it is completed
    loop {
        match run.status {
            RunStatus::Completed => {
                let messages = client
                    .threads()
                    .messages(&thread.id)
                    .list(&[("limit", "10")])
                    .await
                    .context("Failed to list messages")?;

                let newest_message_obj = &messages
                    .data
                    .first()
                    .context("No messages found in thread")?;

                let message_contents = &newest_message_obj.content;
                let mut body = Vec::new();
                for message_content in message_contents {
                    match message_content {
                        MessageContent::Text(text) => {
                            let text_data = &text.text;
                            
                            let mut body_to_push = text_data.value.clone();
                            let annotations = &text_data.annotations;
                            for (ind, annotation) in annotations.iter().enumerate() {
                                if let MessageContentTextAnnotations::FileCitation(obj) = annotation {
                                    body_to_push = body_to_push.replace(
                                        &obj.text,
                                        &format!("[{}]", ind + 1),
                                    );
                                }
                            }
                            body.push(body_to_push);
                            if annotations.len() > 0 {
                                body.push("\n".to_string() + "[ From " + annotations.len().to_string().as_str() + " Sources ]");
                            }
                        }
                        MessageContent::ImageFile(_) | MessageContent::ImageUrl(_) => {
                            body.push("[Image]".to_string());
                        }
                        MessageContent::Refusal(refusal) => {
                            body.push(format!("{}",refusal.refusal));
                        }
                    }
                }
                let event = AssistantUpdate::Message{ content: body.join("\n") };
                socket.send(
                    Message::Text(serde_json::to_string(&event)
                        .context("Failed to serialize 'done thinking' event!")?)
                ).await
                    .context("Failed to send message to client! Error: {e:?}")?;

                break;
            }
            RunStatus::Failed | RunStatus::Cancelled | RunStatus::Expired => {
                let bail_text = match run.status {
                    RunStatus::Failed    => "Run Failed!",
                    RunStatus::Cancelled => "Run Cancelled!",
                    RunStatus::Expired   => "Run Expired!",
                    _ => unreachable!()
                };

                bail!(bail_text);
            }

            RunStatus::Queued     | RunStatus::Cancelling | 
            RunStatus::InProgress | RunStatus::Incomplete | 
            RunStatus::RequiresAction => {
                let status_text = match run.status {
                    RunStatus::Queued     => "Run Queued",
                    RunStatus::Cancelling => "Cancelling...",
                    RunStatus::InProgress => "In Progress...",
                    RunStatus::Incomplete => "Run Incomplete",
                    RunStatus::RequiresAction => "Run Requires Action",
                    _ => unreachable!()
                };

                let event = AssistantUpdate::Waiting{ content: format!("> {}", status_text) };
                socket.send(
                    Message::Text(serde_json::to_string(&event)
                        .context("Failed to serialize 'done thinking' event!")?)
                ).await
                    .context("Failed to send message to client! Error: {e:?}")?;
            }
        }

        // Wait for 1 sec before polling run object again
        sleep(Duration::from_secs(1)).await;

        // Retrieve the run
        run = client.threads().runs(&thread.id).retrieve(&run.id).await?;
    }

    Ok(())
}
pub async fn assistant_entrypoint (
    State(app): State<Arc<AppState>>,
    ws: WebSocketUpgrade
) -> Response {
    ws.on_upgrade(move |socket| handle_socket_helper(app, socket))
}
async fn handle_socket_helper (
    app: Arc<AppState>,
    socket: WebSocket
) -> () {
    if let Err(e) = handle_socket(app, socket).await {
        println!("Failed to handle socket! Error: {e:?}");
    }
}
async fn handle_socket (
    app: Arc<AppState>,
    mut socket: WebSocket
) -> Result<()> {
    let vector_store_id = std::env::var("OPENAI_VECTOR_STORE_ID")
        .context("Couldn't find the OpenAI vector store ID!")?;

    let create_thread_request = CreateThreadRequest {
        messages: None,
        tool_resources: Some(CreateAssistantToolResources {
            code_interpreter: None,
            file_search: Some(CreateAssistantToolFileSearchResources {
                vector_store_ids: Some(vec!(vector_store_id)),
                ..Default::default()
            })
        }),
        ..Default::default()
    };
    let thread = app.openai_client.threads().create(create_thread_request).await?;

    while let Some(msg_result) = socket.recv().await {
        println!("Received message: {msg_result:?}");
        let msg_obj = if let Ok(msg_obj) = msg_result {
            msg_obj
        } else {
            break;
        };

        let msg = match msg_obj {
            Message::Text(text) => text,
            _ => {
                return Ok(());
            }
        };

        if let Err(e) = send_response(
            &app.openai_client,
            &thread,
            &app.openai_assistant,
            &msg,
            &mut socket
        ).await {
            let event = AssistantUpdate::Error { content: format!("> {e}") };
            socket.send(
                Message::Text(serde_json::to_string(&event)
                    .context("Failed to serialize 'error' event!")?)
            ).await
                .context("Failed to send message to client! Error: {e:?}")?;
        }
    }

    // Close the thread
    app.openai_client.threads().delete(&thread.id).await?;

    Ok(())
}