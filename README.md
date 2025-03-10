# S3 Sample

A Rust application demonstrating AWS S3 and CloudFront integration for secure file uploads and downloads.

## Features

- Generate presigned S3 URLs for secure file uploads
- Upload content to S3 using presigned URLs
- Generate CloudFront signed URLs for secure content delivery
- Configurable expiration times for both upload and download URLs

## Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))
- AWS account with S3 bucket and CloudFront distribution set up
- CloudFront key pair for URL signing

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/s3-sample.git
   cd s3-sample
   ```

2. Build the project:
   ```
   cargo build
   ```

## Configuration

1. Create a `.env` file based on the provided `.env.example`:
   ```
   cp .env.example .env
   ```

2. Fill in the required environment variables in the `.env` file:
   ```
   AWS_REGION=your-aws-region
   AWS_ACCESS_KEY_ID=your-access-key
   AWS_SECRET_ACCESS_KEY=your-secret-key
   
   BUCKET_NAME=your-s3-bucket-name
   CLOUDFRONT_DOMAIN=your-cloudfront-domain
   PRIVATE_KEY_PATH=path/to/your/cloudfront-private-key.pem
   CLOUDFRONT_KEY_PAIR_ID=your-cloudfront-key-pair-id
   ```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `AWS_REGION` | AWS region where your S3 bucket is located |
| `AWS_ACCESS_KEY_ID` | Your AWS access key ID |
| `AWS_SECRET_ACCESS_KEY` | Your AWS secret access key |
| `BUCKET_NAME` | Name of your S3 bucket |
| `CLOUDFRONT_DOMAIN` | Your CloudFront distribution domain (e.g., `d1234abcd.cloudfront.net`) |
| `PRIVATE_KEY_PATH` | Path to your CloudFront private key file (`.pem`) |
| `CLOUDFRONT_KEY_PAIR_ID` | Your CloudFront key pair ID |

## Usage

Run the application:

```
cargo run
```

The application will:
1. Generate a presigned S3 upload URL valid for 1 hour
2. Upload a sample text content to S3 using the presigned URL
3. Generate a CloudFront signed URL for downloading the content, valid for 1 hour
4. Print the CloudFront signed URL to the console

## How It Works

### S3 Presigned URLs

The application uses the AWS SDK for Rust to generate presigned URLs for S3 `PutObject` operations. These URLs allow temporary, secure upload access to your S3 bucket without requiring AWS credentials to be shared with the client.

```rust
// Generate a presigned upload URL
let s3_presigned_upload_url = s3::generate_presigned_upload_url(
    &s3_client,
    &config,
    upload_key,
    content_type,
    Duration::from_secs(3600),
).await?;
```

### CloudFront Signed URLs

For secure content delivery, the application generates CloudFront signed URLs using the `cloudfront_sign` crate. These URLs provide temporary access to protected content through your CloudFront distribution.

```rust
// Generate a CloudFront signed URL for downloading
let expiration = Utc::now() + ChronoDuration::hours(1);
let cloudfront_signed_url = s3::generate_signed_url(&config, upload_key, expiration)?;
```

## License

[MIT License](LICENSE)
