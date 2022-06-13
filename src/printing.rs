use std::{io::stdout, time::Duration, thread::sleep};

use crossterm::{
    cursor, execute,
    style::{Color, Print, StyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use serde::Deserialize;

pub enum PrintRow {
    Version = 3,
    NewVersionNotify = 9,
    ClientStatus = 11,
    ActivityStatus = 12,
    Info = 14,
    Error = 15,
    Binds = 17
}

pub fn print_version() {
    #[derive(Deserialize)]
    struct GithubRelease {
        pub tag_name: String,
    }

    let response = minreq::get("https://api.github.com/repos/INikonI/ndrpc/releases/latest")
        .with_header("Accept", "application/vnd.github.v3+json")
        .with_header("User-Agent", "ndrpc")
        .send()
        .unwrap();

    let current_version = env!("CARGO_PKG_VERSION");

    let mut stdout = stdout();
    _ = execute!(
        stdout,
        cursor::MoveTo(37, PrintRow::Version as u16),
        Print(format!("Version {}", current_version).with(Color::Blue)),
    );

    let latest_version = &response.json::<GithubRelease>().unwrap().tag_name[1..];
    if latest_version != current_version {
        _ = execute!(
            stdout,
            cursor::MoveTo(0, PrintRow::NewVersionNotify as u16),
            Print(
                format!(
                    "New version available! https://github.com/INikonI/ndrpc/releases/tag/v{}",
                    latest_version
                )
                .with(Color::Cyan),
            )
        );
    }
}

pub fn print_binds_custom_presence() {
    _ = execute!(
        stdout(),
        cursor::MoveTo(0, PrintRow::Binds as u16),
        Print("Press ".with(Color::Grey)),
        Print("CTRL+R".with(Color::Black).on(Color::Grey)),
        Print(" to reload preset(s)".with(Color::Grey))
    );
}

pub fn print_client_status(text: StyledContent<&str>) {
    _ = execute!(
        stdout(),
        cursor::MoveTo(0, PrintRow::ClientStatus as u16),
        Clear(ClearType::CurrentLine),
        Print("CLIENT STATUS: ".bold()),
        Print(text)
    );
}

pub fn print_activity_status(text: StyledContent<&str>) {
    _ = execute!(
        stdout(),
        cursor::MoveTo(0, PrintRow::ActivityStatus as u16),
        Clear(ClearType::CurrentLine),
        Print("ACTIVITY STATUS: ".bold()),
        Print(text)
    );
}

pub fn print_info(text: StyledContent<&str>) {
    let mut stdout = stdout();
    _ = execute!(
        stdout,
        cursor::MoveTo(0, PrintRow::Info as u16),
        Clear(ClearType::CurrentLine),
        Print("INFO: ".bold()),
        Print(text)
    );
    sleep(Duration::from_secs(3));
    _ = execute!(
        stdout,
        cursor::MoveTo(0, PrintRow::Info as u16),
        Clear(ClearType::CurrentLine)
    );
}

pub fn print_error(text: &str) {
    let mut stdout = stdout();
    _ = execute!(
        stdout,
        cursor::MoveTo(0, PrintRow::Error as u16),
        Clear(ClearType::CurrentLine),
        Print("ERROR: ".with(Color::Red).bold()),
        Print(text)
    );
    sleep(Duration::from_secs(3));
    _ = execute!(
        stdout,
        cursor::MoveTo(0, PrintRow::Error as u16),
        Clear(ClearType::CurrentLine)
    );
}