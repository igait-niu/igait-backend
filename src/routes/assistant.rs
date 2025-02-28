use async_openai::{
    config::OpenAIConfig, types::{
        AssistantObject, CreateAssistantToolFileSearchResources, CreateAssistantToolResources, CreateMessageRequestArgs, CreateRunRequest, CreateThreadRequest, MessageContent, MessageContentTextAnnotations, MessageRole, RunStatus, ThreadObject
    }, Client
};
use anyhow::{Result, Context, bail};
use tokio::time::{Duration, sleep};

enum AssistantUpdate {
    Message(String),
    Error(String),
    Waiting(String)
}

async fn send_response (
    client: &Client<OpenAIConfig>,
    thread: &ThreadObject,
    assistant: &AssistantObject,
    message: &str
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
        //check the status of the run
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
                println!("> Run Completed: {}", body.join("\n"));

                break;
            }
            RunStatus::Failed => {
                bail!("> Run Failed: {:#?}", run);
            }
            RunStatus::Cancelled => {
                bail!("> Run Cancelled");
            }
            RunStatus::Expired => {
                bail!("> Run Expired");
            }

            RunStatus::Queued => {
                println!("> Run Queued");
            }
            RunStatus::Cancelling => {
                println!("> Run Cancelling");
            }
            RunStatus::RequiresAction => {
                println!("> Run Requires Action");
            }
            RunStatus::InProgress => {
                println!("> In Progress ...");
            }
            RunStatus::Incomplete => {
                println!("> Run Incomplete");
            }
        }

        // Wait for 1 sec before polling run object again
        sleep(Duration::from_secs(1)).await;

        // Retrieve the run
        run = client.threads().runs(&thread.id).retrieve(&run.id).await?;
    }

    Ok(())
}