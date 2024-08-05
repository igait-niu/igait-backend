use crate::{helper::lib::User, print_db};

use firebase_rs::*;
use anyhow::{ Context, Result, anyhow };

use super::lib::{Job, JobStatus, JobTaskID};


#[derive( Debug )]
pub struct Database {
    _state: Firebase
}
impl Database {
    pub async fn init () -> Result<Self> {
        Ok(Self {
            _state: Firebase::auth("https://network-technology-project-default-rtdb.firebaseio.com/", &std::env::var("FIREBASE_ACCESS_KEY").context("Missing FIREBASE_ACCESS_KEY! Make sure it's set in your system environment.")?)
                .map_err(|e| anyhow!("{e:?}"))
                .context("Couldn't unwrap the URL while trying to initialize the Firebase wrapper class!")?
                .at("users")
        })
    }
    pub async fn ensure_user (
        &self,
        uid:         &str,
        task_number: JobTaskID
    ) -> Result<()> {
        // Create a path to the user in the database
        let user_handle = self._state.at(uid);

        // Check if the user doesn't exist
        print_db!(task_number, "Verifying user existence...");
        if user_handle.get::<User>().await.is_err() {
            print_db!(task_number, "User doesn't exist, creating new user with UID '{uid}'...");

            // Create a new user
            user_handle.update(&User {
                uid: String::from(uid),
                jobs: Vec::new()
            }).await.map_err(|e| anyhow!("{e:?}")).context("Failed to create a new user while ensuring they existed!")?;
            print_db!(task_number, "Successfully created new user!");
        }

        Ok(())
    }
    pub async fn count_jobs (
        &self,
        uid:         &str,
        task_number: JobTaskID
    ) -> Result<usize> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Then, get the jobs and count them
        if let Ok(jobs) = self._state.at(uid).at("jobs").get::<Vec<Job>>().await {
            return Ok(jobs.len());
        }

        // If there was an error, this means there are no jobs
        Ok(0)
    }
    pub async fn new_job (
        &self,
        uid:         &str,
        job:         Job,
        task_number: JobTaskID
    ) -> Result<()> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Get a handle to the location of the jobs in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs and add the new job
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;
        jobs.push(job);
            
        // Update the user with the new job
        self._state.at(uid).update(&User {
            uid: String::from(uid),
            jobs
        }).await.map_err(|e| anyhow!("{e:?}")).context("Failed to update database with the new job array!")?;

        // Return as successful
        print_db!(task_number, "Added new job!");
        Ok(())
    }
    pub async fn update_status (
        &self, 
        uid:         &str,
        job_id:      usize, 
        status:      JobStatus,
        task_number: JobTaskID
    ) -> Result<()> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;
        
        // Get the user and job handles
        let user_handle = self._state.at(&uid);
        let job_handle = user_handle.at("jobs");

        // Get the jobs as a mutable vector
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;

        // Edit the status
        jobs.get_mut(job_id).ok_or(anyhow!("Job ID does not exist!"))?.status = status.clone();
        
        // Update the user with the modified job array
        user_handle.update(&User {
                uid: String::from(uid),
                jobs
            }).await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to update the user object in the database!")?;

        // Return as successful
        let code = status.code;
        let value = status.value;
        print_db!(task_number, "Updated status successfully to {code:#?} with message {value}!");
        Ok(())
    }
    pub async fn get_status (
        &self, 
        uid:         &str,
        job_id:      usize,
        task_number: JobTaskID
    ) -> Result<JobStatus> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");
        
        // Get the jobs as a mutable vector
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;

        Ok(jobs.get_mut(job_id).ok_or(anyhow!("Job ID does not exist!"))?.status.clone())
    }
    pub async fn get_job (
        &self,
        uid:         &str,
        job_id:      usize,
        task_number: JobTaskID
    ) -> Result<Job> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs as a mutable vector
        let mut jobs = job_handle.get::<Vec<Job>>()
            .await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;

        // Return the job if it exists
        Ok(jobs.get_mut(job_id).ok_or(anyhow!("Job ID does not exist!"))?.clone())
    }
    pub async fn get_all_jobs (
        &self,
        uid:         &str,
        task_number: JobTaskID
    ) -> Result<Vec<Job>> {
        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs as a mutable vector
        job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")
    }
}