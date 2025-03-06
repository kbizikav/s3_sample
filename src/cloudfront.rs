use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use cloudfront_sign;
use std::fs;

use crate::config::Config;

/// Generates a signed URL for CloudFront using the cloudfront_sign crate
///
/// # Arguments
///
/// * `config` - Application configuration
/// * `resource_path` - Path to the resource in CloudFront
/// * `expiration` - When the URL should expire
///
/// # Returns
///
/// * `Result<String>` - The signed URL or error
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
