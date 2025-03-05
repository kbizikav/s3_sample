use anyhow::{Context, Result};
use aws_sdk_s3::{
    primitives::ByteStream,
    Client as S3Client,
    presigning::PresigningConfig,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::time::Duration;
use std::path::Path;

// S3バケット名を設定
const BUCKET_NAME: &str = "my-rust-uploads";
// CloudFrontのドメイン
const CLOUDFRONT_DOMAIN: &str = "d29srixbc05tx1.cloudfront.net";
const CLOUDFRONT_KEY_PAIR_ID: &str = "APKAQ7WPW7R43Y5ZF4NQ";

#[tokio::main]
async fn main() -> Result<()> {
    // AWS SDK設定を読み込み
    let config = aws_config::load_from_env().await;

    // S3クライアントを作成
    let s3_client = S3Client::new(&config);

    // アップロードするファイルのパス
    let file_path = "example.txt";
    let file_key = Path::new(file_path).file_name().unwrap().to_str().unwrap();

    // ファイルをS3にアップロード
    upload_file_to_s3(&s3_client, file_path, file_key).await?;

    // S3のプレサインドURLを生成（有効期限は1時間）
    let s3_presigned_url = generate_presigned_url(&s3_client, file_key, Duration::from_secs(3600)).await?;
    println!("S3プレサインドURL（1時間有効）:");
    println!("{}", s3_presigned_url);

    // CloudFrontのプレサインドURLを生成（有効期限は1時間）
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let cloudfront_signed_url = generate_cloudfront_signed_url(file_key, expiration)?;
    println!("\nCloudFrontプレサインドURL（1時間有効）:");
    println!("{}", cloudfront_signed_url);

    // 通常のCloudFrontのURLを表示
    println!("\n通常のCloudFrontのURL（署名なし）:");
    println!("https://{}/{}", CLOUDFRONT_DOMAIN, file_key);

    Ok(())
}

// S3にファイルをアップロードする関数
async fn upload_file_to_s3(client: &S3Client, file_path: &str, key: &str) -> Result<()> {
    // ファイルを読み込む
    let body = ByteStream::from_path(Path::new(file_path))
        .await
        .context("ファイルの読み込みに失敗しました")?;

    // S3にアップロード
    let resp = client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .body(body)
        .content_type("text/plain")
        .send()
        .await
        .context("S3へのアップロードに失敗しました")?;

    println!("ファイルを正常にアップロードしました: {:?}", resp);
    Ok(())
}

// S3オブジェクトのプレサインドURLを生成する関数
async fn generate_presigned_url(client: &S3Client, key: &str, expiration: Duration) -> Result<String> {
    // プレサイン設定を作成
    let presigning_config = PresigningConfig::builder()
        .expires_in(expiration)
        .build()
        .context("プレサイン設定の作成に失敗しました")?;

    // GetObjectリクエストを作成してプレサインする
    let presigned_request = client
        .get_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .presigned(presigning_config)
        .await
        .context("プレサインドURLの生成に失敗しました")?;

    // URLを文字列として返す
    Ok(presigned_request.uri().to_string())
}

// CloudFrontのプレサインドURLを生成する関数
fn generate_cloudfront_signed_url(resource_path: &str, expiration: DateTime<Utc>) -> Result<String> {
    // CloudFrontのURLを構築
    let url = format!("https://{}/{}", CLOUDFRONT_DOMAIN, resource_path);
    
    // ポリシーを作成
    let policy = format!(
        r#"{{
            "Statement": [
                {{
                    "Resource": "{}",
                    "Condition": {{
                        "DateLessThan": {{
                            "AWS:EpochTime": {}
                        }}
                    }}
                }}
            ]
        }}"#,
        url,
        expiration.timestamp()
    );
    
    // ポリシーをBase64エンコード
    let _policy_base64 = BASE64.encode(policy.as_bytes());
    
    // 署名を作成
    // 注意: 実際のアプリケーションでは、RSA秘密鍵を使用して署名を生成する必要があります
    // ここでは簡略化のためにダミーの署名を返します
    
    // ダミーの署名（実際のアプリケーションでは正しい署名を生成する必要があります）
    let signature = "DUMMY_SIGNATURE_REPLACE_WITH_ACTUAL_IMPLEMENTATION";
    
    // 署名付きURLを構築
    let signed_url = format!(
        "{}?Expires={}&Signature={}&Key-Pair-Id={}",
        url,
        expiration.timestamp(),
        signature,
        CLOUDFRONT_KEY_PAIR_ID
    );
    
    Ok(signed_url)
}
