use std::{fs::read_to_string, io::stdin};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize};

pub fn check_update() {
    #[derive(Deserialize)]
    struct GithubRelease {
        pub tag_name: String,
    }

    let response = minreq::get("https://api.github.com/repos/INikonI/ndrpc/releases/latest")
        .with_header("Accept", "application/vnd.github.v3+json")
        .with_header("User-Agent", "ndrpc")
        .send()
        .unwrap();

    let latest_version = &response.json::<GithubRelease>().unwrap().tag_name[1..];
    if latest_version != env!("CARGO_PKG_VERSION") {
        println!("New version available! https://github.com/INikonI/ndrpc/releases/tag/v{}\n", latest_version);
    }
}

pub fn block_stdin() {
    _ = stdin().read_line(&mut String::new());
}

pub fn parse_yaml_file<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let file_content = read_to_string(path)
        .with_context(|| format!("Failed to read file or file not found \"{}\"", path))?;
    let t: T = serde_yaml::from_str(&file_content).with_context(|| {
        format!(
            "Failed to parse file \"{}\"",
            path.split('/').last().unwrap()
        )
    })?;
    Ok(t)
}
