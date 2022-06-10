mod model;
mod presence;
mod util;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::{thread::sleep, time::Duration};

use crate::{
    model::{
        config::{Config, PresenceKind},
        preset::Preset,
    },
    util::{block_stdin, parse_yaml_file, check_update},
};

fn main() {
    println!(r"            _
           | |
 _ __    __| | _ __  _ __    ___
| '_ \  / _` || '__|| '_ \  / __|
| | | || (_| || |   | |_) || (__
|_| |_| \__,_||_|   | .__/  \___|
                    | |
                    |_|");
    println!("\nCopyright (c) 2022 INikonI\n");

    check_update();

    let config: Config = match parse_yaml_file("config.yaml") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            block_stdin();
            return;
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
        PresenceKind::CustomStatic => {
            let static_preset: Preset = {
                let preset_name: String = match config.static_preset_name {
                    Some(name) => name,
                    None => {
                        eprintln!("Static preset name not found");
                        block_stdin();
                        return;
                    }
                };
                match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                    Ok(preset) => preset,
                    Err(err) => {
                        eprintln!("{}", err);
                        block_stdin();
                        return;
                    }
                }
            };
            presence::custom_static::start(
                drpc,
                static_preset,
                config.with_elapsed_time.unwrap_or_default(),
            );
        }
        PresenceKind::CustomDynamic => {
            let preset_names: Vec<String> = match config.dynamic_preset_names {
                Some(names) => names,
                None => {
                    eprintln!("Dynamic preset names not found");
                    block_stdin();
                    return;
                }
            };
            let presets: Vec<Preset> = {
                let mut presets = Vec::new();
                for preset_name in preset_names {
                    let preset: Preset =
                        match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                            Ok(preset) => preset,
                            Err(err) => {
                                eprintln!("{}", err);
                                block_stdin();
                                return;
                            }
                        };
                    presets.push(preset);
                }
                presets
            };
            presence::custom_dynamic::start(
                drpc,
                presets,
                config.with_elapsed_time.unwrap_or_default(),
            );
        }
        PresenceKind::SystemInfo => {
            let static_preset: Preset = {
                let preset_name: String = match config.static_preset_name {
                    Some(name) => name,
                    None => {
                        eprintln!("Static preset name not found");
                        block_stdin();
                        return;
                    }
                };
                match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                    Ok(preset) => preset,
                    Err(err) => {
                        eprintln!("{}", err);
                        block_stdin();
                        return;
                    }
                }
            };
            presence::system_info::start(drpc, static_preset);
        }
    }
}
