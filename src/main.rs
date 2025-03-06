mod cloudfront;
mod config;
mod s3;

use anyhow::Result;
use aws_sdk_s3::Client as S3Client;
use chrono::{Duration as ChronoDuration, Utc};
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables
    let config = config::Config::from_env()?;

    // Load AWS SDK configuration
    let aws_config = aws_config::load_from_env().await;

    // Create S3 client
    let s3_client = S3Client::new(&aws_config);

    // File to upload
    let file_path = "example.txt";
    let file_key = Path::new(file_path).file_name().unwrap().to_str().unwrap();

    // Upload file to S3
    s3::upload_file(&s3_client, &config, file_path, file_key).await?;

    // Generate S3 presigned URL for downloading (valid for 1 hour)
    let s3_presigned_url =
        s3::generate_presigned_url(&s3_client, &config, file_key, Duration::from_secs(3600))
            .await?;
    println!("S3 Presigned URL for downloading (valid for 1 hour):");
    println!("{}", s3_presigned_url);

    // Generate S3 presigned URL for uploading (valid for 1 hour)
    let content_type = "text/plain"; // Example content type
    let upload_key = "uploaded-content.txt"; // Example key for the uploaded content
    let s3_presigned_upload_url = s3::generate_presigned_upload_url(
        &s3_client,
        &config,
        upload_key,
        content_type,
        Duration::from_secs(3600),
    )
    .await?;
    println!("\nS3 Presigned URL for uploading (valid for 1 hour):");
    println!("{}", s3_presigned_upload_url);
    println!("Content-Type: {}", content_type);
    println!("Note: Use this URL with a PUT request to upload content directly to S3");

    // Generate CloudFront signed URL for downloading (valid for 1 hour)
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let cloudfront_signed_url = cloudfront::generate_signed_url(&config, file_key, expiration)?;
    println!("\nCloudFront Signed URL for downloading (valid for 1 hour):");
    println!("{}", cloudfront_signed_url);

    Ok(())
}
