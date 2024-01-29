use firebase_rs::*;
use sha256::digest;
use crate::{ 
    request::{ Request, StatusCode },
};
use serde::{ Serialize, Deserialize};
use std::fs::{ OpenOptions, File };
use std::io::{ Seek, Write, Read };

#[derive( Serialize, Deserialize, Debug )]
pub struct User {
    pub email: String,
    pub jobs: Vec<Job>
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Job {
    pub id: usize,
    pub age: i16,
    pub ethnicity: String,
    pub gender: char,
    pub height: String,
    pub status: Status,
    pub weight: i16
}
#[derive( Serialize, Deserialize, Clone, Debug )]
pub struct Status {
    pub code: String,
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
    pub async fn new_job (&self, email: String, job: Job) {
        let encoded_email = digest(email.clone());
        let user_handle = self._state.at(&encoded_email);

        if let Ok(user) = user_handle.get::<User>().await {
            let job_handle = user_handle.at("jobs");
            let mut jobs = job_handle.get::<Vec<Job>>().await
                .expect("Failed to get jobs!");
            jobs.push(job);
            
            user_handle.update(&User {
                email,
                jobs
            }).await.expect("Failed to update!");

            println!("{:?}", job_handle.get::<Vec<Job>>().await.unwrap());
        } else {
            user_handle.update(&User {
                email,
                jobs: vec!(job)
            }).await.expect("Failed to update!");
            println!("User doesn't exist!");
        }

        todo!();
    }
    pub fn update_status (&self, status: StatusCode) {
        todo!();
    }
}