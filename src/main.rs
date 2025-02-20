use std::fs;
use std::path::Path;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let releases: Vec<Release> = client
        .get("https://api.github.com/repos/haudeabatid7/gta-6-download-pc/releases")
        .header("User -Agent", "gta_6_download_pc")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let latest_release = &releases[0];
    let asset = &latest_release.assets[0];

    download_file(&asset.browser_download_url, &asset.name).await;
}

async fn download_file(url: &str, filename: &str) {
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    let path = Path::new(filename);
    fs::write(path, &bytes).unwrap();
}