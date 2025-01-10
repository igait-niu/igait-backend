use crate::{helper::lib::User, print_db};

use firebase_rs::*;
use anyhow::{ Context, Result, anyhow };

use super::lib::{Job, JobStatus, JobTaskID};

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
    /// * `task_number` - The task number to print out to the console.
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

    /// Counts the number of jobs a user has.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to count the jobs of.
    /// * `task_number` - The task number to print out to the console.
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
        uid:         &str,
        task_number: JobTaskID
    ) -> Result<usize> {
        print_db!(task_number, "Counting jobs...");

        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

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
    /// * `task_number` - The task number to print out to the console.
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

    /// Updates the status of a job.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to update the job of.
    /// * `job_id` - The ID of the job to update.
    /// * `status` - The new status of the job.
    /// * `task_number` - The task number to print out to the console.
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
        status:      JobStatus,
        task_number: JobTaskID
    ) -> Result<()> {
        print_db!(task_number, "Updating status...");

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
        print_db!(task_number, "Updated status successfully to {code:#?} with message '{value}'!");
        Ok(())
    }

    /// Gets the status of a job.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the job status of.
    /// * `job_id` - The ID of the job to get the status of.
    /// * `task_number` - The task number to print out to the console.
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
        job_id:      usize,
        task_number: JobTaskID
    ) -> Result<JobStatus> {
        print_db!(task_number, "Getting status...");

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

    /// Gets a job given a user ID and a job ID.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the job of.
    /// * `job_id` - The ID of the job to get.
    /// * `task_number` - The task number to print out to the console.
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
        job_id:      usize,
        task_number: JobTaskID
    ) -> Result<Job> {
        print_db!(task_number, "Getting job...");

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

    /// Gets all jobs of a user.
    /// 
    /// # Arguments
    /// * `uid` - The user ID to get the jobs of.
    /// * `task_number` - The task number to print out to the console.
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
        uid:         &str,
        task_number: JobTaskID
    ) -> Result<Vec<Job>> {
        print_db!(task_number, "Getting all jobs...");

        // First double check that the user actually exists
        self.ensure_user(uid, task_number).await.context("Failed to ensure user!")?;

        // Build a path to the job in the database
        let job_handle = self._state.at(uid).at("jobs");

        // Get the jobs as a mutable vector
        job_handle.get::<Vec<Job>>().await
            .map_err(|e| anyhow!("Failed to parse jobs! Error: {e:#?}"))
            .context("Failed to get jobs!")
    }
}