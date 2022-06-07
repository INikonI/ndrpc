use std::{
    io::stdin,
    time::{SystemTime, UNIX_EPOCH},
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::model::config::Config;

pub fn start(mut drpc: DiscordIpcClient, config: Config) {
    let mut activity = Activity::new();

    if let Some(ref details) = config.details {
        activity = activity.details(details);
    }

    if let Some(ref state) = config.state {
        activity = activity.state(state);
    }

    if let Some(ref assets) = config.assets {
        let mut ab = Assets::new();

        if let Some(ref large_image) = assets.large_image {
            ab = ab.large_image(large_image);
        }

        if let Some(ref small_image) = assets.small_image {
            ab = ab.small_image(small_image);
        }

        if let Some(ref large_text) = assets.large_text {
            ab = ab.large_text(large_text);
        }

        if let Some(ref small_text) = assets.small_text {
            ab = ab.small_text(small_text);
        }

        activity = activity.assets(ab);
    }

    if let Some(ref buttons) = config.buttons {
        activity = activity.buttons(
            buttons
                .iter()
                .take(2)
                .map(|b| Button::new(&b.label, &b.url))
                .collect(),
        );
    }

    if Some(true) == config.timestamp {
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
    };

    drop(config);

    loop {
        _ = stdin().read_line(&mut String::new());
    }
}
