# AWS S3 and CloudFront Sample in Rust

This project demonstrates how to interact with Amazon S3 and CloudFront using the AWS SDK for Rust. It provides examples of uploading files to S3 and generating presigned URLs for both S3 and CloudFront.

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install) installed on your system
2. An AWS account with access credentials configured
3. An S3 bucket for file uploads
4. A CloudFront distribution connected to your S3 bucket
5. A CloudFront key pair for signing URLs

## Project Structure

The project is organized into modules:

- `main.rs`: Main application entry point
- `config.rs`: Configuration handling
- `s3.rs`: S3 operations (upload, presigned URLs)
- `cloudfront.rs`: CloudFront operations (signed URLs)

## Configuration

You can configure the application using environment variables or a `.env` file:

```
# AWS S3 Configuration
BUCKET_NAME=your-bucket-name

# AWS CloudFront Configuration
CLOUDFRONT_DOMAIN=your-cloudfront-domain.cloudfront.net
CLOUDFRONT_KEY_PAIR_ID=your-key-pair-id
PRIVATE_KEY_PATH=path/to/your/private-key.pem

# AWS Credentials (optional, can also use ~/.aws/credentials)
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=your_region
```

Copy `.env.example` to `.env` and update with your values.

## Features

This sample demonstrates:

1. Uploading a file to S3
2. Generating a presigned URL for S3 (temporary direct access)
3. Generating a signed URL for CloudFront (temporary access through CDN)

## Running the Sample

Ensure you have a file named `example.txt` in the project root, then:

```
cargo build
cargo run
```

The application will:
1. Upload `example.txt` to your S3 bucket
2. Generate and display a presigned URL for direct S3 access
3. Generate and display a signed URL for CloudFront access

## Error Handling

The application uses the `anyhow` crate for comprehensive error handling, providing clear error messages for various failure scenarios.

## CloudFront Signed URLs

The application demonstrates how to create signed URLs for CloudFront using the `cloudfront_sign` crate. This allows you to provide temporary access to private content distributed through CloudFront.

## Notes

- All URLs generated are valid for 1 hour by default
- The application automatically detects MIME types for uploaded files
- Make sure your CloudFront distribution is properly configured to serve content from your S3 bucket
