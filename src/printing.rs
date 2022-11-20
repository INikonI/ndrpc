use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor, execute,
    style::{Color, Print, StyledContent, Stylize},
    terminal::{Clear, ClearType, SetSize, SetTitle},
};
use serde::Deserialize;

pub enum PrintRow {
    Version = 2,
    Copyright = 4,
    NewVersionNotify = 9,
    ClientStatus = 11,
    ActivityStatus = 12,
    Info = 14,
    Error = 15,
    Binds = 17,
}

pub fn print_header() {
    let mut stdout = stdout();
    _ = execute!(
        stdout,
        SetTitle("ndrpc"),
        SetSize(80, 17),
        cursor::Hide,
        Clear(ClearType::All)
    );
    let ansi_title: Vec<&str> = r"
███    ██  ██████   ██████   ██████    ██████
████   ██  ██   ██  ██   ██  ██   ██  ██
██ ██  ██  ██   ██  ██████   ██████   ██
██  ██ ██  ██   ██  ██   ██  ██       ██
██   ████  ██████   ██   ██  ██        ██████".split('\n').collect();
    for (i, line) in ansi_title.iter().enumerate() {
        _ = execute!(
            stdout,
            cursor::MoveTo(1, i as u16),
            Print(line.with(Color::Blue))
        );
    }
    _ = execute!(
        stdout,
        cursor::MoveTo(50, PrintRow::Copyright as u16),
        Print("Copyright (c) 2022 INikonI".with(Color::Blue).bold()),
        cursor::MoveTo(50, PrintRow::Version as u16),
        Print(format!("Version {}", env!("CARGO_PKG_VERSION")).with(Color::Blue).bold()),
    );
}

pub fn print_new_version_notify() {
    #[derive(Deserialize)]
    struct GithubRelease {
        pub tag_name: String,
    }

    let response = minreq::get("https://api.github.com/repos/INikonI/ndrpc/releases/latest")
        .with_header("Accept", "application/vnd.github.v3+json")
        .with_header("User-Agent", "ndrpc")
        .send()
        .unwrap();

    let latest_version = &response.json::<GithubRelease>().unwrap().tag_name[1..];
    if latest_version != env!("CARGO_PKG_VERSION") {
        _ = execute!(
            stdout(),
            cursor::MoveTo(1, PrintRow::NewVersionNotify as u16),
            Print(
                format!(
                    "New version available! https://github.com/INikonI/ndrpc/releases/tag/v{}",
                    latest_version
                )
                .with(Color::Cyan)
                .bold(),
            )
        );
    }
}

pub fn print_binds_custom_presence() {
    _ = execute!(
        stdout(),
        cursor::MoveTo(1, PrintRow::Binds as u16),
        Print("Press ".with(Color::Grey)),
        Print("CTRL+R".with(Color::Black).on(Color::Grey)),
        Print(" to reload preset(s)".with(Color::Grey))
    );
}

pub fn print_client_status(text: StyledContent<&str>) {
    _ = execute!(
        stdout(),
        cursor::MoveTo(1, PrintRow::ClientStatus as u16),
        Clear(ClearType::CurrentLine),
        Print("CLIENT STATUS: ".bold()),
        Print(text)
    );
}

pub fn print_activity_status(text: StyledContent<&str>) {
    _ = execute!(
        stdout(),
        cursor::MoveTo(1, PrintRow::ActivityStatus as u16),
        Clear(ClearType::CurrentLine),
        Print("ACTIVITY STATUS: ".bold()),
        Print(text)
    );
}

pub fn print_info(text: StyledContent<&str>) {
    let mut stdout = stdout();
    _ = execute!(
        stdout,
        cursor::MoveTo(1, PrintRow::Info as u16),
        Clear(ClearType::CurrentLine),
        Print("INFO: ".bold()),
        Print(text)
    );
    sleep(Duration::from_secs(3));
    _ = execute!(
        stdout,
        cursor::MoveTo(1, PrintRow::Info as u16),
        Clear(ClearType::CurrentLine)
    );
}

pub fn print_error(text: &str) {
    let mut stdout = stdout();
    _ = execute!(
        stdout,
        cursor::MoveTo(1, PrintRow::Error as u16),
        Clear(ClearType::CurrentLine),
        Print("ERROR: ".with(Color::Red).bold()),
        Print(text)
    );
    sleep(Duration::from_secs(3));
    _ = execute!(
        stdout,
        cursor::MoveTo(1, PrintRow::Error as u16),
        Clear(ClearType::CurrentLine)
    );
}
