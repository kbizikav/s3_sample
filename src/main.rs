use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{primitives::ByteStream, Client, Error};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Set up AWS region and credentials
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    println!("=== AWS S3 Sample Application ===");

    // Define bucket and object names for our examples
    let bucket_name = "my-test-bucket-rust-sample";
    let object_key = "sample-object.txt";
    let local_file = "sample-upload.txt";
    let download_file = "sample-download.txt";

    // Create a sample file for upload
    std::fs::write(
        local_file,
        "This is a sample file content for S3 upload test.",
    )
    .expect("Failed to create sample file");

    // List all buckets
    println!("\n1. Listing all buckets:");
    match list_buckets(&client).await {
        Ok(_) => println!("   Buckets listed successfully"),
        Err(e) => {
            println!("   Error listing buckets: {:?}", e);
            return Err(anyhow::anyhow!("Failed to list buckets: {:?}", e));
        }
    }

    // Create a new bucket
    println!("\n2. Creating a new bucket: {}", bucket_name);
    match create_bucket(&client, bucket_name).await {
        Ok(_) => println!("   Bucket created successfully"),
        Err(e) => {
            println!("   Error creating bucket: {:?}", e);
            // Continue with the demo even if bucket creation fails
            // (might already exist from a previous run)
        }
    }

    // Upload an object to the bucket
    println!("\n3. Uploading file to bucket:");
    match upload_object(&client, bucket_name, object_key, local_file).await {
        Ok(_) => println!("   File uploaded successfully"),
        Err(e) => {
            println!("   Error uploading file: {:?}", e);
            return Err(anyhow::anyhow!("Failed to upload file: {:?}", e));
        }
    }

    // List objects in the bucket
    println!("\n4. Listing objects in bucket:");
    list_objects(&client, bucket_name).await?;

    // Download the object
    println!("\n5. Downloading object from bucket:");
    download_object(&client, bucket_name, object_key, download_file).await?;
    println!("   Downloaded to: {}", download_file);

    // Delete the object
    println!("\n6. Deleting object from bucket:");
    delete_object(&client, bucket_name, object_key).await?;

    // Delete the bucket
    println!("\n7. Deleting bucket:");
    delete_bucket(&client, bucket_name).await?;

    // Clean up local files
    let _ = std::fs::remove_file(local_file);
    let _ = std::fs::remove_file(download_file);

    println!("\nAll operations completed successfully!");
    Ok(())
}

// List all S3 buckets
async fn list_buckets(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;

    if let Some(buckets) = resp.buckets() {
        println!("   Found {} buckets:", buckets.len());
        for bucket in buckets {
            println!("   - {}", bucket.name().unwrap_or_default());
        }
    } else {
        println!("   No buckets found");
    }

    Ok(())
}

// Create a new S3 bucket
async fn create_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    let result = client
        .create_bucket()
        .bucket(bucket_name)
        .create_bucket_configuration(
            aws_sdk_s3::types::CreateBucketConfiguration::builder()
                .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::ApNortheast1)
                .build(),
        )
        .send()
        .await;
    match result {
        Ok(_) => {
            println!("   Bucket '{}' created successfully", bucket_name);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

// Upload an object to a bucket
async fn upload_object(
    client: &Client,
    bucket_name: &str,
    object_key: &str,
    file_path: &str,
) -> Result<()> {
    let body = ByteStream::from_path(Path::new(file_path))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;

    client
        .put_object()
        .bucket(bucket_name)
        .key(object_key)
        .body(body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Error uploading file: {}", e))?;

    println!(
        "   File '{}' uploaded as '{}' to bucket '{}'",
        file_path, object_key, bucket_name
    );
    Ok(())
}

// List objects in a bucket
async fn list_objects(client: &Client, bucket_name: &str) -> Result<(), Error> {
    let resp = client.list_objects_v2().bucket(bucket_name).send().await?;

    if let Some(objects) = resp.contents() {
        println!(
            "   Found {} objects in bucket '{}':",
            objects.len(),
            bucket_name
        );
        for obj in objects {
            println!(
                "   - {} ({} bytes)",
                obj.key().unwrap_or_default(),
                obj.size()
            );
        }
    } else {
        println!("   No objects found in bucket '{}'", bucket_name);
    }

    Ok(())
}

// Download an object from a bucket
async fn download_object(
    client: &Client,
    bucket_name: &str,
    object_key: &str,
    file_path: &str,
) -> Result<()> {
    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Error getting object: {}", e))?;

    let data = resp
        .body
        .collect()
        .await
        .map_err(|e| anyhow::anyhow!("Error collecting response data: {}", e))?;
    let bytes = data.into_bytes();

    std::fs::write(file_path, bytes).map_err(|e| anyhow::anyhow!("Error writing file: {}", e))?;

    println!(
        "   Object '{}' downloaded from bucket '{}' to '{}'",
        object_key, bucket_name, file_path
    );
    Ok(())
}

// Delete an object from a bucket
async fn delete_object(client: &Client, bucket_name: &str, object_key: &str) -> Result<()> {
    client
        .delete_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;

    println!(
        "   Object '{}' deleted from bucket '{}'",
        object_key, bucket_name
    );
    Ok(())
}

// Delete a bucket
async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<()> {
    client.delete_bucket().bucket(bucket_name).send().await?;

    println!("   Bucket '{}' deleted", bucket_name);
    Ok(())
}
