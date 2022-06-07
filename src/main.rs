mod model;
mod presence;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::{fs::read_to_string, io::stdin, thread::sleep, time::Duration};

use crate::model::config::{Config, PresenceKind};

fn main() {
    let config: Config = {
        let config_file_content: String = match read_to_string("config.yaml") {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Failed to read config or config not found");
                _ = stdin().read_line(&mut String::new());
                return;
            }
        };

        match serde_yaml::from_str(&config_file_content) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Failed to parse config");
                _ = stdin().read_line(&mut String::new());
                return;
            }
        }
    };

    let mut drpc =
        DiscordIpcClient::new(&config.app_id.to_string()).expect("Failed to create client");
    println!("Connecting client...");
    while drpc.connect().is_err() {
        eprintln!("Failed to connect. Trying to reconnect...");
        sleep(Duration::from_millis(100));
    }
    println!("Client connected!");

    match config.kind {
        PresenceKind::Custom => presence::custom::start(drpc, config),
        PresenceKind::SystemInfo => presence::system_info::start(drpc, config),
    }
}
