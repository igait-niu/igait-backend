use std::time::SystemTime;

use crate::helper::lib::{JobStatusCode, User};

use firebase_rs::*;
use anyhow::{ Context, Result, anyhow };


use super::lib::{Job, JobStatus};

/// A wrapper class on the Firebase database to make it easier to interact with.
#[derive( Debug )]
pub struct Database {
    _state: Firebase
}
impl Database {
    /// Initializes the Firebase wrapper class.
    /// 
    /// # Fails
    /// * If the Firebase URL is invalid
    /// * If the Firebase access key is missing
    /// 
    /// # Returns
    /// * The Firebase wrapper class
    /// 
    /// # Notes
    /// * The Firebase URL is the URL to the Firebase database, not the URL to the Firebase console.
    /// * The Firebase access key is the key that allows you to access the Firebase database.
    /// * The Firebase access key should be stored in the system environment as `FIREBASE_ACCESS_KEY`.
    pub async fn init () -> Result<Self> {
        Ok(Self {
            _state: Firebase::auth("https://network-technology-project-default-rtdb.firebaseio.com/", &std::env::var("FIREBASE_ACCESS_KEY").context("Missing FIREBASE_ACCESS_KEY! Make sure it's set in your system environment.")?)
                .map_err(|e| anyhow!("{e:?}"))
                .context("Couldn't unwrap the URL while trying to initialize the Firebase wrapper class!")?
                .at("users")
        })
    }

    /// Ensures that a user exists in the database.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to ensure exists.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// * If the user can't be updated
    /// 
    /// # Returns
    /// * A successful result if the user exists
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    
    pub async fn ensure_user (
        &self,
        uid: &str
    ) -> Result<()> {
        // Create a path to the user in the database
        let user_handle = self._state.at(uid);

        // Check if the user doesn't exist
        println!("Verifying user existence...");
        if user_handle.get::<User>().await.is_err() {
            println!("User doesn't exist, creating new user with UID '{uid}'...");

            // Create a new user
            user_handle.update(&User {
                uid: String::from(uid),
                jobs: vec!(
                    Job {
                        age: 1,
                        email: String::from("placeholder@placeholder.com"),
                        ethnicity: String::from("placeholder"),
                        height: String::from("placeholder"),
                        sex: 'p',
                        status: JobStatus {
                            code: JobStatusCode::Submitting,
                            value: String::from("placeholder")
                        },
                        timestamp: SystemTime::now(),
                        weight: 1
                    }
                ),
                administrator: false,
            }).await
                .map_err(|e| anyhow!("{e:?}"))
                .context("Failed to create a new user while ensuring they existed!")?;
            println!("Successfully created new user!");
        }

        Ok(())
    }

    /// Counts the number of jobs a user has.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to count the jobs of.
    /// 
    /// # Fails
    /// * If the user can't be ensured
    /// * If the jobs can't be counted
    /// 
    /// # Returns
    /// * The number of jobs the user has
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    /// * This function returns `0` if there are no jobs, but the user exists.
    
    pub async fn count_jobs (
        &self,
        uid:         &str
    ) -> Result<usize> {
        println!("Counting jobs...");

        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;

        // Then, get the jobs and count them
        if let Ok(jobs) = self._state.at(uid).at("jobs").get::<Vec<Job>>().await {
            return Ok(jobs.len());
        }

        // If there was an error, this means there are no jobs
        Ok(0)
    }

    /// Adds a new job to the user's job list.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to add the job to.
    /// * `job` - The job to add to the user's job list.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// * If the jobs can't be updated
    /// 
    /// # Returns
    /// * A successful result if the job was added
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    /// * This function adds the job to the end of the user's job list.
    
    pub async fn new_job (
        &self,
        uid:         &str,
        job:         Job
    ) -> Result<()> {
        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;

        // Get a handle to the location of the jobs in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs and add the new job
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;
        jobs.push(job);
            
        // Get existing user to preserve administrator status
        let existing_user = self._state.at(uid).get::<User>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get existing user!")?;
            
        // Update the user with the new job
        self._state.at(uid).update(&User {
            uid: String::from(uid),
            jobs,
            administrator: existing_user.administrator,
        }).await.map_err(|e| anyhow!("{e:?}")).context("Failed to update database with the new job array!")?;

        // Return as successful
        println!("Added new job!");
        Ok(())
    }

    /// Updates the status of a job.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to update the job of.
    /// * `job_id` - The ID of the job to update.
    /// * `status` - The new status of the job.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// * If the job ID doesn't exist
    ///  
    /// # Returns
    /// * A successful result if the status was updated
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    /// * This function overwrites the status of the job with the new status.
    
    pub async fn update_status (
        &self, 
        uid:         &str,
        job_id:      usize, 
        status:      JobStatus
    ) -> Result<()> {
        println!("Updating status...");

        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;
        
        // Get the user and job handles
        let user_handle = self._state.at(&uid);
        let job_handle = user_handle.at("jobs");

        // Get the jobs as a mutable vector
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;

        // Edit the status
        jobs.get_mut(job_id).ok_or(anyhow!("Job ID does not exist!"))?.status = status.clone();
        
        // Get existing user to preserve administrator status
        let existing_user = user_handle.get::<User>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get existing user!")?;
        
        // Update the user with the modified job array
        user_handle.update(&User {
                uid: String::from(uid),
                jobs,
                administrator: existing_user.administrator,
            }).await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to update the user object in the database!")?;

        // Return as successful
        let code = status.code;
        let value = status.value;
        println!("Updated status successfully to {code:#?} with message '{value}'!");
        Ok(())
    }

    /// Gets the status of a job.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the job status of.
    /// * `job_id` - The ID of the job to get the status of.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// * If the job ID doesn't exist
    /// 
    /// # Returns
    /// * The status of the job
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    
    pub async fn _get_status (
        &self, 
        uid:         &str,
        job_id:      usize
    ) -> Result<JobStatus> {
        println!("Getting status...");

        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");
        
        // Get the jobs as a mutable vector
        let mut jobs = job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("{e:?}"))
            .context("Failed to get jobs!")?;

        Ok(jobs.get_mut(job_id).ok_or(anyhow!("Job ID does not exist!"))?.status.clone())
    }

    /// Gets a job given a user ID and a job ID.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the job of.
    /// * `job_id` - The ID of the job to get.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// * If the job ID doesn't exist
    /// 
    /// # Returns
    /// * The job
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    
    pub async fn get_job (
        &self,
        uid:         &str,
        job_id:      usize
    ) -> Result<Job> {
        println!("Getting job...");

        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;

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

    /// Gets all jobs of a user.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the jobs of.
    /// 
    /// # Fails
    /// * If the user doesn't exist and can't be created
    /// 
    /// # Returns
    /// * The jobs of the user
    /// 
    /// # Notes
    /// * This function creates a new user if the user doesn't exist.
    
    pub async fn get_all_jobs (
        &self,
        uid:         &str
    ) -> Result<Vec<Job>> {
        println!("Getting all jobs...");

        // First double check that the user actually exists
        self.ensure_user(uid).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs as a mutable vector
        job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("Failed to parse jobs! Error: {e:#?}"))
            .context("Failed to get jobs!")
    }
}