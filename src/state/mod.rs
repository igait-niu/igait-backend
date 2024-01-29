use crate::{ 
    database::{ Database, Status, Job },
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
    pub async fn new() -> Self {
        let db = Database::init().await;
        db.new_job(
            String::from("fakeemaildeletelater@gmail.com"),
            Job {
                id: 80,
                age: 227,
                ethnicity: String::from("caucasian"),
                gender: 'm',
                height: String::from("5'11"),
                status: Status {
                    code: String::from("hiya"),
                    value: String::from(":3-"),
                },
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