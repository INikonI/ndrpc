use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::model::preset::Preset;

pub fn set(drpc: &mut DiscordIpcClient, preset: &Preset, start_timestamp: &Option<i64>) -> bool {
    let mut activity = Activity::new();

    if let Some(ref details) = preset.details {
        let details = details.trim();
        if !details.is_empty() {
            activity = activity.details(details);
        }
    }

    if let Some(ref state) = preset.state {
        let state = state.trim();
        if !state.is_empty() {
            activity = activity.state(state);
        }
    }

    if let Some(ref assets) = preset.assets {
        let mut ab = Assets::new();

        if let Some(ref large_image) = assets.large_image {
            let large_image = large_image.trim();
            if !large_image.is_empty() {
                ab = ab.large_image(large_image);
            }
        }

        if let Some(ref large_text) = assets.large_text {
            let large_text = large_text.trim();
            if !large_text.is_empty() {
                ab = ab.large_text(large_text);
            }
        }

        if let Some(ref small_image) = assets.small_image {
            let small_image = small_image.trim();
            if !small_image.is_empty() {
                ab = ab.small_image(small_image);
            }
        }

        if let Some(ref small_text) = assets.small_text {
            let small_text = small_text.trim();
            if !small_text.is_empty() {
                ab = ab.small_text(small_text);
            }
        }

        activity = activity.assets(ab);
    }

    if let Some(ref buttons) = preset.buttons {
        let buttons: Vec<(&str, &str)> = buttons
            .iter()
            .take(2)
            .map(|b| (b.label.trim(), b.url.trim()))
            .filter(|b| !b.0.is_empty() && !b.1.is_empty())
            .collect();
        if !buttons.is_empty() {
            activity = activity.buttons(buttons.iter().map(|b| Button::new(b.0, b.1)).collect());
        }
    }

    if let Some(timestamp) = start_timestamp {
        activity = activity.timestamps(Timestamps::new().start(*timestamp));
    }

    drpc.set_activity(activity).is_ok()
}
