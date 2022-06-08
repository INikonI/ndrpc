use std::time::{SystemTime, UNIX_EPOCH};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::{model::preset::Preset, util::wait_input};

pub fn start(mut drpc: DiscordIpcClient, preset: Preset, with_elapsed_time: bool) {
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
        activity = activity.timestamps(
            Timestamps::new().start(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    * 1000,
            ),
        );
    }

    match drpc.set_activity(activity) {
        Ok(_) => println!("Activity setted"),
        Err(_) => eprintln!("Activity set failed"),
    }

    drop(preset);

    loop {
        wait_input();
    }
}
