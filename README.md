# S3 and CloudFront URL Generator

This Rust application demonstrates how to:

1. Upload files to Amazon S3
2. Generate presigned URLs for downloading content from S3
3. Generate presigned URLs for uploading content to S3
4. Generate CloudFront signed URLs for downloading content

## Prerequisites

- Rust and Cargo installed
- AWS credentials configured (either in `~/.aws/credentials` or as environment variables)
- CloudFront key pair for generating signed URLs

## Configuration

The application uses the following environment variables:

```
# AWS Credentials
AWS_REGION=your-region
AWS_ACCESS_KEY_ID=your-access-key
AWS_SECRET_ACCESS_KEY=your-secret-key

# S3 Configuration
BUCKET_NAME=your-bucket-name

# CloudFront Configuration
CLOUDFRONT_DOMAIN=your-cloudfront-domain
CLOUDFRONT_KEY_PAIR_ID=your-key-pair-id
PRIVATE_KEY_PATH=path/to/your/private-key.pem
```

You can set these in a `.env` file in the project root.

## Running the Application

```bash
cargo run
```

This will:
1. Upload the `example.txt` file to S3
2. Generate and display an S3 presigned URL for downloading the file
3. Generate and display an S3 presigned URL for uploading content
4. Display the regular CloudFront URL for the file
5. Generate and display a CloudFront signed URL for downloading the file

## Uploading Content Using the Presigned URL

You can use the generated S3 presigned upload URL to upload content directly to S3. The `upload-example.sh` script demonstrates how to do this using curl:

1. Run the application to generate a presigned upload URL
2. Copy the URL and update the `PRESIGNED_URL` variable in `upload-example.sh`
3. Run the script:

```bash
./upload-example.sh
```

## Notes on CloudFront for Uploading

CloudFront is primarily designed for content distribution, not for receiving uploads. For uploading content, it's recommended to use S3 presigned URLs directly (as shown above).

If you need to upload through CloudFront, you would need to configure CloudFront with an S3 origin and set up appropriate behaviors and origin access controls to allow PUT/POST requests.
