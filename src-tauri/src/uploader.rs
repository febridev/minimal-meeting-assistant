use std::path::Path;
use reqwest::multipart;
use serde::Deserialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct RouterResponse {
    content: String,
}

pub async fn upload_to_9router(
    file_path: &Path,
    api_key: &str,
    api_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let file_content = fs::read(file_path)?;
    let part = multipart::Part::bytes(file_content)
        .file_name(file_path.file_name().unwrap().to_string_lossy().to_string())
        .mime_str("audio/wav")?;

    let form = multipart::Form::new().part("file", part);

    let res = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?;

    if res.status().is_success() {
        let response_data: RouterResponse = res.json().await?;
        Ok(response_data.content)
    } else {
        Err(format!("Upload failed with status: {}", res.status()).into())
    }
}

pub fn save_summary(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = dirs::document_dir().ok_or("Could not find documents directory")?;
    let summaries_dir = home_dir.join("Meeting-Summaries");
    fs::create_dir_all(&summaries_dir)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let filename = format!("summary_{}.md", timestamp);
    let file_path = summaries_dir.join(filename);

    fs::write(&file_path, content)?;
    Ok(file_path.to_string_lossy().to_string())
}
