use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::fs::read_to_string;

pub fn parse_yaml_file<T>(path: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let file_content = read_to_string(path)
        .with_context(|| format!("Failed to read file or file not found \"{}\"", path))?;
    serde_yaml::from_str(&file_content).with_context(||
        format!(
            "Failed to parse file \"{}\"",
            path.split('/').last().unwrap()
        )
    )
}
