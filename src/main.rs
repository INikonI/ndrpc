mod model;
mod presence;
mod printing;
mod util;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Stylize},
    terminal::enable_raw_mode,
};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use model::{
    config::{Config, PresenceKind},
    preset::Preset,
};
use printing::{
    print_activity_status, print_binds_custom_presence, print_client_status, print_error,
    print_header, print_info, print_new_version_notify,
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use util::parse_yaml_file;

fn main() {
    {
        use sysinfo::{ProcessRefreshKind, RefreshKind, SystemExt};
        let sys = sysinfo::System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::new()),
        );
        if sys.processes_by_name("ndrpc").count() > 1 {
            return;
        }
    }

    enable_raw_mode().unwrap();
    print_header();
    print_new_version_notify();

    let config: Config = match parse_yaml_file("config.yaml") {
        Ok(config) => config,
        Err(err) => {
            print_error(&err.to_string());
            read().unwrap();
            return;
        }
    };

    let mut drpc =
        DiscordIpcClient::new(&config.app_id.to_string()).expect("Failed to create client");
    print_client_status("Connecting...".with(Color::Yellow));
    while drpc.connect().is_err() {
        sleep(Duration::from_millis(100));
    }
    print_client_status("Connected".with(Color::Green));

    let start_timestamp: Option<i64> = if config.with_elapsed_time.unwrap_or_default() {
        Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                * 1000,
        )
    } else {
        None
    };

    match config.kind {
        PresenceKind::CustomStatic => {
            let static_preset: Preset = {
                let preset_name: &str = match config.static_preset_name {
                    Some(ref name) => name,
                    None => {
                        print_error("Static preset name not found");
                        read().unwrap();
                        return;
                    }
                };
                match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                    Ok(preset) => preset,
                    Err(err) => {
                        print_error(&err.to_string());
                        read().unwrap();
                        return;
                    }
                }
            };
            if presence::custom::set(&mut drpc, &static_preset, &start_timestamp) {
                print_activity_status("Updated".with(Color::Green));
            } else {
                print_activity_status("Update failed".with(Color::Red));
            }
            drop(static_preset);
            print_binds_custom_presence();
            loop {
                match read().unwrap() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('r'),
                        modifiers: KeyModifiers::CONTROL,
                    }) => {
                        let static_preset: Preset = {
                            let preset_name: &str = match config.static_preset_name {
                                Some(ref name) => name,
                                None => {
                                    print_error("Static preset name not found");
                                    continue;
                                }
                            };
                            match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                                Ok(preset) => preset,
                                Err(err) => {
                                    print_error(&err.to_string());
                                    continue;
                                }
                            }
                        };
                        presence::custom::set(&mut drpc, &static_preset, &start_timestamp);
                        print_info("Static preset reloaded".with(Color::Yellow))
                    }
                    _ => (),
                };
            }
        }
        PresenceKind::CustomDynamic => {
            let preset_names: &Vec<String> = match config.dynamic_preset_names {
                Some(ref names) => names,
                None => {
                    print_error("Dynamic preset names not found");
                    read().unwrap();
                    return;
                }
            };
            let mut presets: Vec<Preset> = Vec::new();
            for preset_name in preset_names {
                let preset: Preset =
                    match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                        Ok(preset) => preset,
                        Err(err) => {
                            print_error(&err.to_string());
                            read().unwrap();
                            return;
                        }
                    };
                presets.push(preset);
            }

            let mut update_fails: u8 = 0;
            let mut cycle = presets.iter().cycle();
            print_binds_custom_presence();
            loop {
                let preset = cycle.next().unwrap();
                if presence::custom::set(&mut drpc, preset, &start_timestamp) {
                    print_activity_status("Updated".with(Color::Green));
                } else if update_fails > 1 {
                    update_fails = 0;
                    print_client_status("Reconnecting...".with(Color::Yellow));
                    while drpc.reconnect().is_err() {
                        sleep(Duration::from_millis(100));
                    }
                    print_client_status("Connected".with(Color::Green));
                } else {
                    update_fails += 1;
                    print_activity_status("Update failed".with(Color::Red));
                }

                if poll(Duration::from_millis(4010)).unwrap() {
                    match read().unwrap() {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('r'),
                            modifiers: KeyModifiers::CONTROL,
                        }) => {
                            print_info("Dynamic presets reloading...".with(Color::Yellow));
                            presets = Vec::new();
                            for preset_name in preset_names {
                                let preset: Preset = match parse_yaml_file(&format!(
                                    "./presets/{}.yaml",
                                    preset_name
                                )) {
                                    Ok(preset) => preset,
                                    Err(err) => {
                                        print_error(&err.to_string());
                                        continue;
                                    }
                                };
                                presets.push(preset);
                            }
                            cycle = presets.iter().cycle();
                            print_info("Dynamic presets reloaded".with(Color::Yellow));
                        }
                        _ => (),
                    }
                }
            }
        }
        PresenceKind::SystemInfo => {
            let static_preset: Preset = {
                let preset_name: String = match config.static_preset_name {
                    Some(name) => name,
                    None => {
                        print_error("Static preset name not found");
                        read().unwrap();
                        return;
                    }
                };
                match parse_yaml_file(&format!("./presets/{}.yaml", preset_name)) {
                    Ok(preset) => preset,
                    Err(err) => {
                        print_error(&err.to_string());
                        read().unwrap();
                        return;
                    }
                }
            };
            print_activity_status("Loading...".with(Color::Yellow));
            presence::system_info::start(drpc, static_preset);
        }
    }
}
