use crate::{ 
    database::Database,
    inference,

    request::{ StatusCode },
    Arc, Mutex
};

#[derive(Debug)]
pub struct AppState {
    working: bool,
    db: Database
}
impl AppState {
    pub fn new() -> Self {
        Self {
            working: false,
            db: Database::init()
        }
    }

    pub async fn work_queue(s: Arc<Mutex<AppState>>) {
        if s.lock().await.working {
            return;
        }
        s.lock().await.working = true;

        
    }
}