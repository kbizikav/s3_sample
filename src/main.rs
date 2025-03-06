mod cloudfront;
mod config;
mod s3;

use anyhow::Result;
use aws_sdk_cloudfront::Client as CloudFrontClient;
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
    let file_key = Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Upload file to S3
    s3::upload_file(&s3_client, &config, file_path, file_key).await?;

    // Generate S3 presigned URL (valid for 1 hour)
    let s3_presigned_url = s3::generate_presigned_url(
        &s3_client,
        &config,
        file_key,
        Duration::from_secs(3600),
    )
    .await?;
    println!("S3 Presigned URL (valid for 1 hour):");
    println!("{}", s3_presigned_url);

    // Create CloudFront client (not used in this example but kept for future use)
    let _cloudfront_client = CloudFrontClient::new(&aws_config);

    // Display regular CloudFront URL
    println!("\nRegular CloudFront URL (unsigned):");
    println!("https://{}/{}", config.cloudfront_domain, file_key);

    // Generate CloudFront signed URL (valid for 1 hour)
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let cloudfront_signed_url = cloudfront::generate_signed_url(&config, file_key, expiration)?;
    println!("\nCloudFront Signed URL (valid for 1 hour):");
    println!("{}", cloudfront_signed_url);

    Ok(())
}
