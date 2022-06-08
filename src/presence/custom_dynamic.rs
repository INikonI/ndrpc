use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::model::preset::Preset;

pub fn start(mut drpc: DiscordIpcClient, presets: Vec<Preset>, with_elapsed_time: bool) {
    let timestamp: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        * 1000;

    let mut update_fails: u8 = 0;

    for preset in presets.iter().cycle() {
        let mut activity = Activity::new();

        if let Some(ref details) = preset.details {
            activity = activity.details(details);
        }

        if let Some(ref state) = preset.state {
            activity = activity.state(state);
        }

        if let Some(ref assets) = preset.assets {
            let mut ab = Assets::new();

            if let Some(ref large_image) = assets.large_image {
                ab = ab.large_image(large_image);
            }

            if let Some(ref large_text) = assets.large_text {
                ab = ab.large_text(large_text);
            }

            if let Some(ref small_image) = assets.small_image {
                ab = ab.small_image(small_image);
            }

            if let Some(ref small_text) = assets.small_text {
                ab = ab.small_text(small_text);
            }

            activity = activity.assets(ab);
        }

        if let Some(ref buttons) = preset.buttons {
            activity = activity.buttons(
                buttons
                    .iter()
                    .take(2)
                    .map(|b| Button::new(&b.label, &b.url))
                    .collect(),
            );
        }

        if with_elapsed_time {
            activity = activity.timestamps(Timestamps::new().start(timestamp));
        }

        match drpc.set_activity(activity) {
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
                    eprintln!("Activity update failed");
                }
            }
        }

        sleep(Duration::from_millis(4010));
    }
}
