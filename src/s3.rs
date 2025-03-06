use anyhow::{Context, Result};
use aws_sdk_s3::{presigning::PresigningConfig, primitives::ByteStream, Client as S3Client};
use std::path::Path;
use std::time::Duration;

use crate::config::Config;

/// Uploads a file to S3 bucket
///
/// # Arguments
///
/// * `client` - The S3 client
/// * `config` - Application configuration
/// * `file_path` - Path to the file to upload
/// * `key` - Key (name) to use for the file in S3
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn upload_file(
    client: &S3Client,
    config: &Config,
    file_path: &str,
    key: &str,
) -> Result<()> {
    // Read the file
    let body = ByteStream::from_path(Path::new(file_path))
        .await
        .context(format!("Failed to read file: {}", file_path))?;

    // Determine content type based on file extension
    let content_type = mime_guess::from_path(file_path)
        .first_or_octet_stream()
        .to_string();

    // Upload to S3
    let resp = client
        .put_object()
        .bucket(&config.bucket_name)
        .key(key)
        .body(body)
        .content_type(content_type)
        .send()
        .await
        .context("Failed to upload file to S3")?;

    println!("Successfully uploaded file: {:?}", resp);
    Ok(())
}

/// Generates a presigned URL for an S3 object
///
/// # Arguments
///
/// * `client` - The S3 client
/// * `config` - Application configuration
/// * `key` - Key (name) of the file in S3
/// * `expiration` - How long the URL should be valid
///
/// # Returns
///
/// * `Result<String>` - The presigned URL or error
pub async fn generate_presigned_url(
    client: &S3Client,
    config: &Config,
    key: &str,
    expiration: Duration,
) -> Result<String> {
    // Create presigning configuration
    let presigning_config = PresigningConfig::builder()
        .expires_in(expiration)
        .build()
        .context("Failed to create presigning configuration")?;

    // Create and presign GetObject request
    let presigned_request = client
        .get_object()
        .bucket(&config.bucket_name)
        .key(key)
        .presigned(presigning_config)
        .await
        .context("Failed to generate presigned URL")?;

    // Return URL as string
    Ok(presigned_request.uri().to_string())
}
