use crate::{ 
    database,
    inference,

    request::{ Request, Status },
    Arc, Mutex
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct AppState {
    pub queue: VecDeque<usize>,
    working: bool,

    pub db: database::Database
}
impl AppState {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            working: false,
            db: database::Database::new()
        }
    }

    pub async fn work_queue(s: Arc<Mutex<AppState>>) {
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
                .status = Status::Processing;

            let confidence = inference::run_inference(id).await;

            s.lock().await
                .db
                .get(id)
                .expect("Unreachable")
                .status = Status::Complete(confidence);
            println!("[Finished {id}]");
        }
        s.lock().await.working = false;
    }
}