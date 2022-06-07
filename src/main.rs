mod utils;

use chrono::{prelude::Local, Timelike};
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sys_info::loadavg as cpu_usage;
use sysinfo::{CpuExt, RefreshKind, SystemExt};
use utils::prompt;

#[cfg(target_os = "windows")]
fn main() {
    use std::collections::HashMap;
    use sysinfo::CpuRefreshKind;
    use wmi::{COMLibrary, Variant, WMIConnection};

    // info providers
    let wmi_con = WMIConnection::new(COMLibrary::new().unwrap().into()).unwrap();
    let mut sysinfo_system = sysinfo::System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new())
            .with_memory(),
    );

    // precompute some info
    let max_freq: f64 = {
        let result: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query("SELECT MaxClockSpeed FROM Win32_Processor")
            .unwrap();
        if let Variant::UI4(val) = result
            .into_iter()
            .next()
            .unwrap()
            .get("MaxClockSpeed")
            .unwrap()
        {
            *val as f64
        } else {
            0.0
        }
    };
    let physical_cores: u8 = sysinfo_system
        .physical_core_count()
        .expect("Failed to get physical core count") as u8;
    let logical_cores: u8 = sysinfo_system.cpus().len() as u8;
    let cpu_brand: String = sysinfo_system.global_cpu_info().brand().trim().to_owned();
    let total_memory: f64 = (sysinfo_system.total_memory() as f64 / 1024_f64 / 1024_f64).round();

    // Discord RPC init
    let (app_id, large_image) = {
        let input = prompt("Enter app id (optional):").unwrap();
        if input.len() > 16 && input.len() < 26 && input.parse::<u64>().is_ok() {
            (input, prompt("Enter image key or url (optional):").unwrap())
        } else {
            ("983347731823210567".to_owned(), "shark".to_owned())
        }
    };
    let mut drpc = DiscordIpcClient::new(&app_id).expect("Failed to create client");

    println!("Connecting client...");
    while drpc.connect().is_err() {
        eprintln!("Failed to connect. Trying to reconnect...");
        sleep(Duration::from_millis(100));
    }
    println!("Client connected!");

    // main loop
    let mut update_fails: u8 = 0;
    loop {
        let used_memory: f64 = (sysinfo_system.used_memory() as f64 / 1024_f64 / 1024_f64).round();
        let current_freq: f64 = {
            let result: Vec<HashMap<String, Variant>> = wmi_con
                .raw_query("SELECT PercentProcessorPerformance FROM Win32_PerfFormattedData_Counters_ProcessorInformation WHERE Name LIKE \"_Total\"")
                .unwrap();
            let percent_perfomance: f64 = if let Variant::UI8(val) = result
                .first()
                .unwrap()
                .get("PercentProcessorPerformance")
                .unwrap()
            {
                *val as f64
            } else {
                0.0
            };
            percent_perfomance * max_freq / 100_f64 / 1000_f64
        };

        match drpc.set_activity(
            Activity::new()
                .details(&format!(
                    "CPU: {:.0}% | RAM: {}/{} GB",
                    cpu_usage().expect("Failed to get cpu usage").one * 100_f64,
                    used_memory,
                    &total_memory
                ))
                .state(&format!(
                    "{:.2} GHz | {}/{} Cores | {}",
                    current_freq, &physical_cores, &logical_cores, &cpu_brand
                ))
                .assets(if large_image.is_empty() {
                    Assets::new()
                } else {
                    Assets::new().large_image(&large_image)
                })
                .timestamps(
                    Timestamps::new().start(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64
                            - (Local::now().time().num_seconds_from_midnight() as i64 * 1000),
                    ),
                ),
        ) {
            Ok(_) => println!("Activity updated"),
            Err(_) => {
                if update_fails > 2 {
                    update_fails = 0;
                    println!("Trying to reconnect...");
                    while drpc.reconnect().is_err() {
                        eprintln!("Failed to reconnect. Trying again...");
                        sleep(Duration::from_millis(100));
                    }
                    println!("Client reconnected!");
                } else {
                    update_fails += 1;
                    eprintln!("Activity update failed")
                }
            },
        }

        sleep(Duration::from_millis(4010));
        sysinfo_system.refresh_memory();
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // info providers
    let mut sysinfo_system = sysinfo::System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new())
            .with_memory(),
    );

    // precompute some info
    let physical_cores: u8 = sysinfo_system
        .physical_core_count()
        .expect("Failed to get physical core count") as u8;
    let logical_cores: u8 = sysinfo_system.cpus().len() as u8;
    let cpu_brand: String = sysinfo_system.global_cpu_info().brand().trim().to_owned();
    let total_memory: f64 = (sysinfo_system.total_memory() as f64 / 1024_f64 / 1024_f64).round();

    // Discord RPC init
    let (app_id, large_image) = {
        let input = prompt("Enter app id (optional):").unwrap();
        if input.len() > 16 && input.len() < 26 && input.parse::<u64>().is_ok() {
            (input, prompt("Enter image key or url (optional):").unwrap())
        } else {
            ("983347731823210567".to_owned(), "shark".to_owned())
        }
    };
    let mut drpc = DiscordIpcClient::new(&app_id).expect("Failed to create client");

    println!("Connecting client...");
    while drpc.connect().is_err() {
        eprintln!("Failed to connect. Trying to reconnect...");
        sleep(Duration::from_millis(100));
    }
    println!("Client connected!");

    // main loop
    let mut update_fails: u8 = 0;
    loop {
        let used_memory: f64 = (sysinfo_system.used_memory() as f64 / 1024_f64 / 1024_f64).round();
        let current_freq: f64 = sysinfo_system.global_cpu_info().frequency() as f64 / 1000_f64;

        match drpc.set_activity(
            Activity::new()
                .details(&format!(
                    "CPU: {:.0}% | RAM: {}/{} GB",
                    cpu_usage().expect("Failed to get cpu usage").one * 100.0,
                    used_memory,
                    &total_memory
                ))
                .state(&format!(
                    "{:.2} GHz | {}/{} Cores | {}",
                    current_freq, &physical_cores, &logical_cores, &cpu_brand
                ))
                .assets(if large_image.is_empty() {
                    Assets::new()
                } else {
                    Assets::new().large_image(&large_image)
                })
                .timestamps(
                    Timestamps::new().start(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64
                            - (Local::now().time().num_seconds_from_midnight() as i64 * 1000),
                    ),
                ),
        ) {
            Ok(_) => println!("Activity updated"),
            Err(_) => {
                if update_fails > 2 {
                    update_fails = 0;
                    println!("Trying to reconnect...");
                    while drpc.reconnect().is_err() {
                        eprintln!("Failed to reconnect. Trying again...");
                        sleep(Duration::from_millis(100));
                    }
                    println!("Client reconnected!");
                } else {
                    update_fails += 1;
                    eprintln!("Activity update failed")
                }
            },
        }

        sleep(Duration::from_millis(4010));
        sysinfo_system.refresh_memory();
    }
}
