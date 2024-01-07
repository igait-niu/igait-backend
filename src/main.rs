use axum::{
    routing::{ get, post },
    extract::{ Path, State },
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::{convert::Infallible, io};

#[derive(Debug)]
enum RequestState {
    Submitting,
    Submitted,
    SubmissionErr,
    Queue(usize),
    Processing,
    PorcessingErr,
    Complete(f32)
}
#[derive(Debug)]
struct Request {
    id: usize,
    state: RequestState
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
                state: RequestState::Submitting
            }
        );
        id
    }
}

#[derive(Debug)]
struct AppState {
    queue: Vec<usize>,
    db: Database
}
impl AppState {
    fn new() {

    }
}

async fn upload(State(app): State<Arc<Mutex<AppState>>>) -> Result<String, String> {
    let id = app
        .lock().await
        .db
        .new_entry();
    
    println!("[New Entry: ID {id}]");
    Ok(id.to_string())
}
async fn status(Path(id): Path<i32>) -> Result<String, String> {
    Err(String::from("todo"))
}
#[tokio::main]
async fn main() {
    let state: Arc<Mutex<AppState>> = Arc::new(
        Mutex::new(
            AppState { 
                queue: Vec::new(),
                db: Database::new()
            }
        )
    );

    // build our application with a single route
    let api_v1 = Router::new()
        .route("/status/:id", get(status))
        .route("/upload", post(upload).get(upload) )// GET IS TEMPORARY
        .with_state(state);

    let app = Router::new()
        .nest("/api/v1", api_v1);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}