use crate::{ 
    database,
    inference,

    request::{ Status },
    Arc, Mutex
};

#[derive(Debug)]
pub struct AppState {
    working: bool,

    pub db: database::Database
}
impl AppState {
    pub fn new() -> Self {
        Self {
            working: false,
            db: database::Database::from_path("data/db.json")
        }
    }

    pub async fn update_status(s: &Arc<Mutex<AppState>>, id: usize, status: Status) {
        println!("[Updating {} to {:?}]", id, status);
        s.lock().await
            .db
            .get(id)
            .expect(format!("Job ID {} Not Found!", id).as_str())
            .status = status;
        s.lock().await
            .db
            .save();
    }
    pub async fn work_queue(s: Arc<Mutex<AppState>>) {
        if s.lock().await.working {
            return;
        }
        s.lock().await.working = true;

        while let Some(id) = {
            let id_option = s.lock().await
                .db
                .next_queue();

            id_option
        } {
            Self::update_status(&s, id, Status::Processing).await;

            let result = inference::run_inference(id).await
                .unwrap_or_else(|err| Status::InferenceErr(err));

            Self::update_status(&s, id, result).await;
        }
        s.lock().await.working = false;
    }
}