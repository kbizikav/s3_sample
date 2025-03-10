use aws_sdk_s3::{error::SdkError, presigning::PresigningConfig, Client as S3Client};
use serde::Deserialize;
use std::fs;
use std::io;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, S3Error>;

#[derive(Debug, Deserialize)]
pub struct S3Config {
    pub bucket_name: String,
    pub cloudfront_domain: String,
    pub cloudfront_key_pair_id: String,
    pub private_key_path: String,
}

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("Failed to create presigning configuration: {0}")]
    PresigningConfig(String),

    #[error("Failed to generate presigned upload URL: {0}")]
    PresignedUrlGeneration(String),

    #[error("Failed to check if object exists: {0}")]
    ObjectExistenceCheck(String),

    #[error("Failed to read private key file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to generate CloudFront signed URL: {0}")]
    CloudFrontSigning(String),
}

pub async fn generate_presigned_upload_url(
    client: &S3Client,
    config: &S3Config,
    key: &str,
    content_type: &str,
    expiration: Duration,
) -> Result<String> {
    let presigning_config = PresigningConfig::builder()
        .expires_in(expiration)
        .build()
        .map_err(|e| S3Error::PresigningConfig(e.to_string()))?;

    let presigned_request = client
        .put_object()
        .bucket(&config.bucket_name)
        .key(key)
        .content_type(content_type)
        .presigned(presigning_config)
        .await
        .map_err(|e| S3Error::PresignedUrlGeneration(e.to_string()))?;

    Ok(presigned_request.uri().to_string())
}

pub async fn check_object_exists(client: &S3Client, config: &S3Config, key: &str) -> Result<bool> {
    match client
        .head_object()
        .bucket(&config.bucket_name)
        .key(key)
        .send()
        .await
    {
        Ok(_) => Ok(true),
        Err(err) => {
            if let SdkError::ServiceError(service_err) = &err {
                if service_err.err().is_not_found() {
                    return Ok(false);
                }
            }
            Err(S3Error::ObjectExistenceCheck(format!("{:?}", err)))
        }
    }
}

pub fn generate_signed_url(
    config: &S3Config,
    resource_path: &str,
    expiration: Duration,
) -> Result<String> {
    let url = format!("https://{}/{}", config.cloudfront_domain, resource_path);

    // Read the private key file
    let private_key_data = fs::read_to_string(&config.private_key_path)?;

    let options = cloudfront_sign::SignedOptions {
        key_pair_id: config.cloudfront_key_pair_id.clone(),
        private_key: private_key_data,
        date_less_than: chrono::Utc::now().timestamp() as u64 + expiration.as_secs(),
        ..Default::default()
    };

    let signed_url = cloudfront_sign::get_signed_url(&url, &options)
        .map_err(|e| S3Error::CloudFrontSigning(format!("{:?}", e)))?;

    Ok(signed_url)
}
