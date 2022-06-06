use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use precord_core::Features;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sys_info::loadavg as cpu_usage;
use sysinfo::{ProcessorExt, RefreshKind, SystemExt};
use chrono::{prelude::Local, Timelike};

fn main() {
    let precord_system = precord_core::System::new(Features::CPU_FREQUENCY, None).unwrap();
    let mut sysinfo_system =
        sysinfo::System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());

    let physical_cores = sysinfo_system
        .physical_core_count()
        .expect("Failed to get physical core count");
    let logical_cores = sysinfo_system.processors().len();
    let cpu_brand = sysinfo_system.global_processor_info().brand().to_owned();
    let total_memory = (sysinfo_system.total_memory() as f64 / 1024_f64 / 1024_f64).round();

    let mut drpc = DiscordIpcClient::new("983347731823210567").expect("Failed to create client");

    println!("Connecting client...");
    while drpc.connect().is_err() {
        eprintln!("Failed to connect. Trying to reconnect...");
        thread::sleep(Duration::from_millis(100));
    }
    println!("Client connected!");

    loop {
        let freqs = precord_system
            .cpus_frequency()
            .expect("Failed to get cpus frequency");
        let used_memory = (sysinfo_system.used_memory() as f64 / 1024_f64 / 1024_f64).round();
        // let date_now = Local::now();
        match drpc.set_activity(
            Activity::new()
                .details(&format!(
                    "CPU: {:.0}% | RAM: {}/{} GB",
                    cpu_usage().unwrap().one * 100.0,
                    used_memory,
                    &total_memory
                ))
                .state(&format!(
                    "{:.2} GHz | {}/{} Cores | {}",
                    freqs.iter().sum::<f32>() / freqs.len() as f32 / 1024_f32,
                    &physical_cores,
                    &logical_cores,
                    &cpu_brand
                ))
                .assets(Assets::new().large_image("shark"))
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
            Err(_) => eprintln!("Activity update failed"),
        }

        thread::sleep(Duration::from_millis(4001));
        sysinfo_system.refresh_memory();
    }
}
