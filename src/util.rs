use std::{fs::read_to_string, io::stdin};

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;

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
