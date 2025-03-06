use anyhow::Result;
use std::env;

pub struct Config {
    pub bucket_name: String,
    pub cloudfront_domain: String,
    pub cloudfront_key_pair_id: String,
    pub private_key_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Try to load from .env file if it exists
        let _ = dotenv::dotenv();

        Ok(Self {
            bucket_name: env::var("BUCKET_NAME").unwrap_or_else(|_| "my-rust-uploads".to_string()),
            cloudfront_domain: env::var("CLOUDFRONT_DOMAIN")
                .unwrap_or_else(|_| "d29srixbc05tx1.cloudfront.net".to_string()),
            cloudfront_key_pair_id: env::var("CLOUDFRONT_KEY_PAIR_ID")
                .unwrap_or_else(|_| "APKAQ7WPW7R43Y5ZF4NQ".to_string()),
            private_key_path: env::var("PRIVATE_KEY_PATH")
                .unwrap_or_else(|_| "keys/pk-APKAQ7WPW7R43Y5ZF4NQ.pem".to_string()),
        })
    }
}
