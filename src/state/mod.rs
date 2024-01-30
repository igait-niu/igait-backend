use crate::{ 
    database::{ Database, Status, Job },
    inference,

    request::{ StatusCode },
    Arc, Mutex
};
use std::time::SystemTime;

#[derive(Debug)]
pub struct AppState {
    working: bool,
    pub db: Database
}
impl AppState {
    pub async fn new() -> Self {
        let db = Database::init().await;
        db.new_job(
            String::from("fakeemaildeletelater@gmail.com"),
            Job {
                age: 227,
                ethnicity: String::from("caucasian"),
                gender: 'm',
                height: String::from("5'11"),
                status: Status {
                    code: StatusCode::Submitting,
                    value: String::from(":3-"),
                },
                timestamp: SystemTime::now(),
                weight: 135
            }
        ).await;
        Self {
            working: false,
            db,
        }
    }

    pub async fn work_queue(s: Arc<Mutex<AppState>>) {
        if s.lock().await.working {
            return;
        }
        s.lock().await.working = true;

        
    }
}