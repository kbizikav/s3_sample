use anyhow::{Context, Result};
use aws_sdk_cloudfront::Client as CloudFrontClient;
use aws_sdk_s3::{
    primitives::ByteStream,
    Client as S3Client,
};
use mime_guess::from_path;
use std::path::Path;

// S3バケット名とCloudFrontディストリビューションIDを設定
const BUCKET_NAME: &str = "my-rust-uploads";
const CLOUDFRONT_DISTRIBUTION_ID: &str = "E3DBRIHOOQCFIE";

#[tokio::main]
async fn main() -> Result<()> {
    // AWS SDK設定を読み込み
    let config = aws_config::load_from_env().await;

    // S3クライアントとCloudFrontクライアントを作成
    let s3_client = S3Client::new(&config);
    let cloudfront_client = CloudFrontClient::new(&config);

    // アップロードするファイルのパス
    let file_path = "example.txt";
    let file_key = Path::new(file_path).file_name().unwrap().to_str().unwrap();

    // ファイルをS3にアップロード
    upload_file_to_s3(&s3_client, file_path, file_key).await?;

    // CloudFrontキャッシュを無効化（必要な場合）
    invalidate_cloudfront_cache(&cloudfront_client, &[file_key]).await?;

    // CloudFrontのURLを表示
    let cloudfront_domain = "your-distribution-domain.cloudfront.net";
    println!("ファイルは以下のURLからアクセスできます:");
    println!("https://{}/{}", cloudfront_domain, file_key);

    Ok(())
}

// S3にファイルをアップロードする関数
async fn upload_file_to_s3(client: &S3Client, file_path: &str, key: &str) -> Result<()> {
    // ファイルを読み込む
    let body = ByteStream::from_path(Path::new(file_path))
        .await
        .context("ファイルの読み込みに失敗しました")?;

    // Content-Typeを推測
    let content_type = from_path(file_path).first_or_octet_stream().to_string();

    // S3にアップロード
    let resp = client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .body(body)
        .content_type(content_type)
        .send()
        .await
        .context("S3へのアップロードに失敗しました")?;

    println!("ファイルを正常にアップロードしました: {:?}", resp);
    Ok(())
}

// CloudFrontのキャッシュを無効化する関数
async fn invalidate_cloudfront_cache(client: &CloudFrontClient, paths: &[&str]) -> Result<()> {
    // パスの前に「/」を付ける
    let paths_with_slash: Vec<String> = paths.iter().map(|p| format!("/{}", p)).collect();

    // パスの設定を作成
    let paths = aws_sdk_cloudfront::types::Paths::builder()
        .quantity(paths_with_slash.len() as i32)
        .set_items(Some(paths_with_slash))
        .build();

    // 無効化バッチを作成
    let invalidation_batch = aws_sdk_cloudfront::types::InvalidationBatch::builder()
        .caller_reference(format!("invalidation-{}", chrono::Utc::now().timestamp()))
        .paths(paths)
        .build();

    // 無効化リクエストを作成
    let resp = client
        .create_invalidation()
        .distribution_id(CLOUDFRONT_DISTRIBUTION_ID)
        .invalidation_batch(invalidation_batch)
        .send()
        .await
        .context("CloudFrontキャッシュの無効化に失敗しました")?;

    println!("キャッシュ無効化リクエストを送信しました: {:?}", resp);
    Ok(())
}
