use firebase_rs::*;
use sha256::digest;
use crate::{ 
    request::{ StatusCode },
    print::*
};
use serde::{ Serialize, Deserialize};
use std::time::SystemTime;

#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    pub email: String,
    pub jobs: Vec<Job>
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Job {
    pub age: i16,
    pub ethnicity: String,
    pub gender: char,
    pub height: String,
    pub status: Status,
    pub timestamp: SystemTime,
    pub weight: i16
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Status {
    pub code: StatusCode,
    pub value: String,
}

#[derive( Debug )]
pub struct Database {
    _state: Firebase
}
impl Database {
    pub async fn init () -> Self {
        Self {
            _state: Firebase::auth("https://network-technology-project-default-rtdb.firebaseio.com/", "***REMOVED***")
                .unwrap()
                .at("users")
        }
    }
    pub async fn count_jobs (&self, email: String ) -> usize {
        let encoded_email = digest(email.clone());
        let user_handle = self._state.at(&encoded_email);

        if let Ok(_user) = user_handle.get::<User>().await {
            let job_handle = user_handle.at("jobs");

            if let Ok(jobs) = job_handle.get::<Vec<Job>>().await {
                return jobs.len();
            }
        }
        return 0;
    }
    pub async fn new_job (&self, email: String, job: Job) {
        let encoded_email = digest(email.clone());
        let user_handle = self._state.at(&encoded_email);

        if let Ok(_user) = user_handle.get::<User>().await {
            let job_handle = user_handle.at("jobs");
            let mut jobs = job_handle.get::<Vec<Job>>().await
                .expect("Failed to get jobs!");

            jobs.push(job);
            
            user_handle.update(&User {
                email,
                jobs
            }).await.expect("Failed to update!");
            print_db("Added new job!");
        } else {
            print_db(&format!("User doesn't exist, creating new user with email '{email}'..."));

            user_handle.update(&User {
                email,
                jobs: vec!(job)
            }).await.expect("Failed to update!");
        }
    }
    pub async fn update_status (&self, user_id: String, job_id: usize, status: Status) {
        let user_handle = self._state.at(&user_id);

        if let Ok(user) = user_handle.get::<User>().await {
            let email = user.email;
            let job_handle = user_handle.at("jobs");
            let mut jobs = job_handle.get::<Vec<Job>>().await
                .expect("Failed to get jobs!");

            if let Some(job_ref) = jobs.get_mut(job_id) {
                (*job_ref).status = status.clone();
            } else {
                println!("----------\nWARNING! FILES OUT OF SYNC!\n\nIF YOU SEE THIS MESSAGE, NETWORKING IS MISCONFIGURED! THIS SHOULD BE ADDRESSED IMMEDIAETLY!\n----------");
            }
            
            user_handle.update(&User {
                email,
                jobs
            }).await.expect("Failed to update!");

            print_db(&format!("Updated status successfully to {:?} with message {:?}!", status.code, status.value));
        } else {
            println!("User doesn't exist!");
        }
    }
}