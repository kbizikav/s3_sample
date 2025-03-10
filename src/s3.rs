use anyhow::{Context, Result};
use aws_sdk_s3::{presigning::PresigningConfig, Client as S3Client};
use std::time::Duration;

use crate::config::Config;

use chrono::{DateTime, Utc};
use cloudfront_sign;
use std::fs;

pub fn generate_signed_url(
    config: &Config,
    resource_path: &str,
    expiration: DateTime<Utc>,
) -> Result<String> {
    // Build the CloudFront URL
    let url = format!("https://{}/{}", config.cloudfront_domain, resource_path);

    // Read the private key file
    let private_key_data = fs::read_to_string(&config.private_key_path).context(format!(
        "Failed to read private key file: {}",
        config.private_key_path
    ))?;

    // Create SignedOptions
    let mut options = cloudfront_sign::SignedOptions::default();
    options.key_pair_id = config.cloudfront_key_pair_id.clone();
    options.private_key = private_key_data;
    options.date_less_than = expiration.timestamp() as u64;

    // Generate signed URL
    let signed_url = cloudfront_sign::get_signed_url(&url, &options)
        .map_err(|e| anyhow::anyhow!("Failed to generate CloudFront signed URL: {:?}", e))?;

    Ok(signed_url)
}

pub async fn generate_presigned_upload_url(
    client: &S3Client,
    config: &Config,
    key: &str,
    content_type: &str,
    expiration: Duration,
) -> Result<String> {
    // Create presigning configuration
    let presigning_config = PresigningConfig::builder()
        .expires_in(expiration)
        .build()
        .context("Failed to create presigning configuration")?;

    // Create and presign PutObject request
    let presigned_request = client
        .put_object()
        .bucket(&config.bucket_name)
        .key(key)
        .content_type(content_type)
        .presigned(presigning_config)
        .await
        .context("Failed to generate presigned upload URL")?;

    // Return URL as string
    Ok(presigned_request.uri().to_string())
}
