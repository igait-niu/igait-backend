use axum::{
    routing::{ get, post },
    extract::{ Path, State },
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::VecDeque;
use std::{convert::Infallible, io};

#[derive(Debug)]
enum RequestStatus {
    Submitting,
    Submitted,
    SubmissionErr,
    Queue(usize),
    Processing,
    ProcessingErr,
    Complete(f32)
}
#[derive(Debug)]
struct Request {
    id: usize,
    status: RequestStatus
}

#[derive(Debug)]
struct Database {
    entries: Vec<Request>
}
impl Database {
    fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }
    fn from_path() -> Self {
        todo!();
    }

    fn new_entry(&mut self) -> usize {
        let id = (*self).entries.len();
        (*self).entries.push( 
            Request { 
                id,
                status: RequestStatus::Submitting
            }
        );
        id
    }
    fn get(&mut self, index: usize) -> Option<&mut Request> {
        (*self).entries
            .get_mut(index)
    }
}

#[derive(Debug)]
struct AppState {
    queue: VecDeque<usize>,
    working: bool,

    db: Database
}
impl AppState {
    fn new() {

    }

    async fn work_queue(s: Arc<Mutex<AppState>>) {
        if s.lock().await.working {
            return;
        }
        s.lock().await.working = true;

        loop {
            if s.lock().await.queue.len() == 0 {
                break;
            }
            let id = s.lock().await.queue
                .pop_front()
                .expect("Unreachable");

            println!("[Starting on {id}]");
            s.lock().await
                .db
                .get(id)
                .expect("Unreachable")
                .status = RequestStatus::Processing;

            // stuff
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

            s.lock().await
                .db
                .get(id)
                .expect("Unreachable")
                .status = RequestStatus::Complete(20202.20f32);
            println!("[Finished {id}]");
        }
        s.lock().await.working = false;
    }
}

async fn upload(State(app): State<Arc<Mutex<AppState>>>) -> Result<String, String> {
    // Create ID
    let id = app
        .lock().await
        .db
        .new_entry();

    // Save file
    // write_to_file(name: id);
    app.lock().await
        .db
        .get(id).ok_or("ID not found. Request may have been deleted.")?
        .status = RequestStatus::Submitted;

    // Update queue
    app.lock().await
        .queue
        .push_back(id);
    {
        let mut app_lock = app.lock().await;
        app_lock.db
            .get(id).ok_or("ID not found. Request may have been deleted.")?
            .status = RequestStatus::Queue(app_lock.queue.len());
    }
    tokio::spawn(AppState::work_queue(app.clone()));
    
    println!("[New Entry: ID {id}]");
    Ok(id.to_string())
}
async fn status(Path(id): Path<usize>, State(app): State<Arc<Mutex<AppState>>>) -> String {
    app
        .lock().await
        .db
        .get(id)
        .and_then(|request| Some(format!("{:?}", request.status)))
        .unwrap_or(String::from("Unable to find ID!"))
}
#[tokio::main]
async fn main() {
    let state: Arc<Mutex<AppState>> = Arc::new(
        Mutex::new(
            AppState { 
                queue: VecDeque::new(),
                working: false,
                db: Database::new()
            }
        )
    );

    let api_v1 = Router::new()
        .route("/status/:id", get(status))
        .route("/upload", post(upload).get(upload) )// GET IS TEMPORARY
        .with_state(state);

    let app = Router::new()
        .nest("/api/v1", api_v1);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}