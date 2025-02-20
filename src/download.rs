use reqwest::Client;
use std::fs;
use std::path::Path;

pub async fn download_file(url: &str, filename: &str) {
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let bytes = response.bytes().await.unwrap();
    let path = Path::new(filename);
    fs::write(path, &bytes).unwrap();
}