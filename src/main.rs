mod s3;

use anyhow::Result;
use aws_sdk_s3::Client as S3Client;
use chrono::{Duration as ChronoDuration, Utc};
use s3::{check_object_exists, S3Config};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = envy::from_env::<S3Config>()?;
    let aws_config = aws_config::load_from_env().await;
    let s3_client = S3Client::new(&aws_config);

    let content_type = "application/text";
    let upload_key = format!("uploaded-content-{}.txt", uuid::Uuid::new_v4());
    let s3_presigned_upload_url = s3::generate_presigned_upload_url(
        &s3_client,
        &config,
        &upload_key,
        content_type,
        Duration::from_secs(3600),
    )
    .await?;

    // check if the object exists
    let result = check_object_exists(&s3_client, &config, &upload_key).await?;
    println!("Object exists: {}", result);

    // put content to the presigned URL
    let content = "Hello, world!";
    let _ = reqwest::Client::new()
        .put(&s3_presigned_upload_url)
        .header("Content-Type", content_type)
        .body(content.to_string())
        .send()
        .await?;

    let result = check_object_exists(&s3_client, &config, &upload_key).await?;
    println!("Object exists: {}", result);

    // Generate CloudFront signed URL for downloading (valid for 1 hour)
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let cloudfront_signed_url = s3::generate_signed_url(&config, &upload_key, expiration)?;
    println!("\nCloudFront Signed URL for downloading (valid for 1 hour):");
    println!("{}", cloudfront_signed_url);

    Ok(())
}
