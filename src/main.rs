mod s3;
use s3::{S3Client, S3Config};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = envy::from_env::<S3Config>()?;
    let aws_config = aws_config::load_from_env().await;
    let s3_client = S3Client::new(aws_config, config);

    let content_type = "application/text";
    let upload_key = format!("uploaded-content-{}.txt", uuid::Uuid::new_v4());
    let s3_presigned_upload_url = s3_client
        .generate_presigned_upload_url(
            &upload_key,
            content_type,
            Duration::from_secs(10), // valid for 10sec
        )
        .await?;

    // check if the object exists
    let result = s3_client.check_object_exists(&upload_key).await?;
    println!("Object exists: {}", result);

    // put content to the presigned URL
    let content = "Hello, world!";
    let _ = reqwest::Client::new()
        .put(&s3_presigned_upload_url)
        .header("Content-Type", content_type)
        .body(content.to_string())
        .send()
        .await?;

    let result = s3_client.check_object_exists(&upload_key).await?;
    println!("Object exists: {}", result);

    let cloudfront_signed_url =
        s3_client.generate_signed_url(&upload_key, Duration::from_secs(10))?;
    println!("cloud front url: {}", cloudfront_signed_url);

    // download the object
    let downloaded_content = reqwest::get(&cloudfront_signed_url).await?.text().await?;
    println!("Downloaded content: {}", downloaded_content);

    Ok(())
}
