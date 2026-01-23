//! Storage utilities for Firebase Storage / GCS access.

use anyhow::{Context, Result};
use std::path::Path;

#[cfg(feature = "microservice")]
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{
        download::Range,
        get::GetObjectRequest,
        upload::{Media, UploadObjectRequest, UploadType},
        delete::DeleteObjectRequest,
    },
};

/// Configuration for storage access.
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Firebase Storage bucket name
    pub bucket: String,
    
    /// Optional endpoint override (for local emulator)
    pub endpoint: Option<String>,
}

impl StorageConfig {
    /// Creates a new StorageConfig from environment variables.
    /// 
    /// Reads:
    /// - `FIREBASE_STORAGE_BUCKET` (required)
    /// - `STORAGE_ENDPOINT` (optional, for emulator)
    pub fn from_env() -> Result<Self> {
        let bucket = std::env::var("FIREBASE_STORAGE_BUCKET")
            .unwrap_or_else(|_| "network-technology-project.firebasestorage.app".to_string());
        
        let endpoint = std::env::var("STORAGE_ENDPOINT").ok();
        
        Ok(Self { bucket, endpoint })
    }

    /// Returns the full GCS URI for a storage key.
    pub fn gcs_uri(&self, key: &str) -> String {
        format!("gs://{}/{}", self.bucket, key)
    }
}

/// A wrapper around the GCS client for Firebase Storage operations.
#[cfg(feature = "microservice")]
#[derive(Clone)]
pub struct StorageClient {
    client: Client,
    bucket: String,
}

#[cfg(feature = "microservice")]
impl StorageClient {
    /// Creates a new StorageClient from environment configuration.
    /// 
    /// Uses Application Default Credentials (ADC) for authentication.
    /// Set `GOOGLE_APPLICATION_CREDENTIALS` to your service account JSON.
    pub async fn new() -> Result<Self> {
        let config = StorageConfig::from_env()?;
        Self::with_config(config).await
    }

    /// Creates a new StorageClient with a specific configuration.
    pub async fn with_config(config: StorageConfig) -> Result<Self> {
        let client_config = ClientConfig::default()
            .with_auth()
            .await
            .context("Failed to configure GCS auth (check GOOGLE_APPLICATION_CREDENTIALS)")?;
        
        let client = Client::new(client_config);
        
        Ok(Self {
            client,
            bucket: config.bucket,
        })
    }

    /// Uploads bytes to a storage key.
    pub async fn upload(&self, key: &str, data: Vec<u8>, _content_type: Option<&str>) -> Result<()> {
        let upload_type = UploadType::Simple(Media::new(key.to_string()));
        
        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data,
                &upload_type,
            )
            .await
            .context(format!("Failed to upload object: {}", key))?;
        
        Ok(())
    }

    /// Downloads bytes from a storage key.
    pub async fn download(&self, key: &str) -> Result<Vec<u8>> {
        let data = self.client
            .download_object(
                &GetObjectRequest {
                    bucket: self.bucket.clone(),
                    object: key.to_string(),
                    ..Default::default()
                },
                &Range::default(),
            )
            .await
            .context(format!("Failed to download object: {}", key))?;
        
        Ok(data)
    }

    /// Deletes an object from storage.
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.client
            .delete_object(&DeleteObjectRequest {
                bucket: self.bucket.clone(),
                object: key.to_string(),
                ..Default::default()
            })
            .await
            .context(format!("Failed to delete object: {}", key))?;
        
        Ok(())
    }

    /// Returns the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Returns a GCS URI for a key.
    pub fn gcs_uri(&self, key: &str) -> String {
        format!("gs://{}/{}", self.bucket, key)
    }
}

#[cfg(feature = "microservice")]
impl std::fmt::Debug for StorageClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageClient")
            .field("bucket", &self.bucket)
            .finish()
    }
}

/// Constructs storage paths for job files.
pub struct StoragePaths;

impl StoragePaths {
    /// Returns the base path for a job's files.
    /// Format: `jobs/{job_id}/`
    pub fn job_base(job_id: &str) -> String {
        format!("jobs/{}/", job_id)
    }

    /// Returns the path for a stage's output directory.
    /// Format: `jobs/{job_id}/stage_{n}/`
    pub fn stage_dir(job_id: &str, stage: u8) -> String {
        format!("jobs/{}/stage_{}/", job_id, stage)
    }

    /// Returns the path for the original uploaded files.
    /// Format: `jobs/{job_id}/stage_0/`
    pub fn uploads_dir(job_id: &str) -> String {
        format!("jobs/{}/stage_0/", job_id)
    }

    /// Returns the full path for an uploaded front video.
    pub fn upload_front_video(job_id: &str, extension: &str) -> String {
        format!("jobs/{}/stage_0/front.{}", job_id, extension)
    }

    /// Returns the full path for an uploaded side video.
    pub fn upload_side_video(job_id: &str, extension: &str) -> String {
        format!("jobs/{}/stage_0/side.{}", job_id, extension)
    }

    /// Returns the path for the final results archive.
    pub fn results_archive(job_id: &str) -> String {
        format!("jobs/{}/stage_7/results.zip", job_id)
    }

    /// Extracts the job_id from a storage path.
    /// Assumes format: `jobs/{job_id}/...`
    pub fn extract_job_id(path: &str) -> Option<&str> {
        let path = path.strip_prefix("jobs/")?;
        path.split('/').next()
    }
}

/// Helper trait for working with storage keys in stage services.
pub trait StorageKeyExt {
    /// Returns the filename portion of the storage key.
    fn filename(&self) -> Option<&str>;
    
    /// Returns the extension of the storage key.
    fn extension(&self) -> Option<&str>;
}

impl StorageKeyExt for str {
    fn filename(&self) -> Option<&str> {
        Path::new(self).file_name()?.to_str()
    }

    fn extension(&self) -> Option<&str> {
        Path::new(self).extension()?.to_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_paths() {
        assert_eq!(
            StoragePaths::job_base("user123_5"),
            "jobs/user123_5/"
        );
        
        assert_eq!(
            StoragePaths::stage_dir("user123_5", 1),
            "jobs/user123_5/stage_1/"
        );
        
        assert_eq!(
            StoragePaths::upload_front_video("user123_5", "mp4"),
            "jobs/user123_5/stage_0/front.mp4"
        );
        
        assert_eq!(
            StoragePaths::extract_job_id("jobs/user123_5/stage_1/front.mp4"),
            Some("user123_5")
        );
    }

    #[test]
    fn test_storage_key_ext() {
        assert_eq!("jobs/test/video.mp4".filename(), Some("video.mp4"));
        assert_eq!("jobs/test/video.mp4".extension(), Some("mp4"));
    }
}
