//! Storage utilities for AWS S3 access.

#[allow(unused_imports)]
use anyhow::{Context, Result};
use std::path::Path;

#[cfg(feature = "microservice")]
use aws_sdk_s3::{
    Client,
    presigning::PresigningConfig,
    primitives::ByteStream,
};

/// Configuration for storage access.
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// AWS S3 bucket name
    pub bucket: String,
    
    /// AWS region
    pub region: String,
}

impl StorageConfig {
    /// Creates a new StorageConfig from environment variables.
    /// 
    /// Reads:
    /// - `AWS_S3_BUCKET` (defaults to "igait-storage")
    /// - `AWS_REGION` (defaults to "us-east-2")
    pub fn from_env() -> Result<Self> {
        let bucket = std::env::var("AWS_S3_BUCKET")
            .unwrap_or_else(|_| "igait-storage".to_string());
        
        let region = std::env::var("AWS_REGION")
            .unwrap_or_else(|_| "us-east-2".to_string());
        
        Ok(Self { bucket, region })
    }

    /// Returns the full S3 URI for a storage key.
    pub fn s3_uri(&self, key: &str) -> String {
        format!("s3://{}/{}", self.bucket, key)
    }
}

/// A wrapper around the AWS S3 client for storage operations.
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
    /// Uses AWS credentials from environment variables or IAM roles.
    /// Set `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` for authentication.
    pub async fn new() -> Result<Self> {
        let config = StorageConfig::from_env()?;
        Self::with_config(config).await
    }

    /// Creates a new StorageClient with a specific configuration.
    pub async fn with_config(config: StorageConfig) -> Result<Self> {
        let aws_config = aws_config::load_from_env().await;
        let client = Client::new(&aws_config);
        
        Ok(Self {
            client,
            bucket: config.bucket,
        })
    }

    /// Uploads bytes to a storage key.
    pub async fn upload(&self, key: &str, data: Vec<u8>, content_type: Option<&str>) -> Result<()> {
        let body = ByteStream::from(data);
        
        let mut request = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body);
        
        if let Some(ct) = content_type {
            request = request.content_type(ct);
        }
        
        request
            .send()
            .await
            .context(format!("Failed to upload object: {}", key))?;
        
        Ok(())
    }

    /// Downloads bytes from a storage key.
    pub async fn download(&self, key: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .context(format!("Failed to download object: {}", key))?;
        
        let data = response.body.collect()
            .await
            .context("Failed to read object body")?;
        
        Ok(data.into_bytes().to_vec())
    }

    /// Deletes an object from storage.
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .context(format!("Failed to delete object: {}", key))?;
        
        Ok(())
    }

    /// Lists all object keys that begin with the given prefix.
    pub async fn list_by_prefix(&self, prefix: &str) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);

            if let Some(token) = &continuation_token {
                request = request.continuation_token(token);
            }

            let response = request
                .send()
                .await
                .context(format!("Failed to list objects with prefix: {}", prefix))?;

            for object in response.contents() {
                if let Some(key) = object.key() {
                    keys.push(key.to_string());
                }
            }

            match response.next_continuation_token() {
                Some(token) => continuation_token = Some(token.to_string()),
                None => break,
            }
        }

        Ok(keys)
    }

    /// Deletes all objects whose keys begin with the given prefix.
    ///
    /// Returns the number of objects deleted.
    pub async fn delete_by_prefix(&self, prefix: &str) -> Result<usize> {
        let keys = self.list_by_prefix(prefix).await?;
        let count = keys.len();

        for key in &keys {
            self.delete(key).await
                .context(format!("Failed to delete object during prefix deletion: {}", key))?;
        }

        Ok(count)
    }

    /// Returns the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Returns an S3 URI for a key.
    pub fn s3_uri(&self, key: &str) -> String {
        format!("s3://{}/{}", self.bucket, key)
    }

    /// Generates a presigned GET URL for a storage key.
    ///
    /// The returned URL allows unauthenticated downloads for the
    /// specified duration (no AWS credentials required by the caller).
    pub async fn presign_download(
        &self,
        key: &str,
        expires_in: std::time::Duration,
    ) -> Result<String> {
        let presigning_config = PresigningConfig::expires_in(expires_in)
            .context("Invalid presigning duration")?;

        let presigned = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .context(format!("Failed to presign download for key: {}", key))?;

        Ok(presigned.uri().to_string())
    }

    /// Lists all objects under `prefix` and returns a presigned URL for each.
    ///
    /// Returns a vec of `(key, presigned_url)` pairs.
    pub async fn list_and_presign(
        &self,
        prefix: &str,
        expires_in: std::time::Duration,
    ) -> Result<Vec<(String, String)>> {
        let keys = self.list_by_prefix(prefix).await?;
        let mut results = Vec::with_capacity(keys.len());

        for key in keys {
            let url = self.presign_download(&key, expires_in).await?;
            results.push((key, url));
        }

        Ok(results)
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

    /// Returns the full path for a stage output front video.
    pub fn stage_front_video(job_id: &str, stage: u8, extension: &str) -> String {
        format!("jobs/{}/stage_{}/front.{}", job_id, stage, extension)
    }

    /// Returns the full path for a stage output side video.
    pub fn stage_side_video(job_id: &str, stage: u8, extension: &str) -> String {
        format!("jobs/{}/stage_{}/side.{}", job_id, stage, extension)
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
