use std::{process::Command, str::from_utf8};

use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Torrent {
    id: String,
    name: String,
    state: String,
    eta: String,
    percent: String,
    download_speed: String,
    upload_speed: String,
    size: String,
    downloaded: String,
}

pub fn get_torrent_ids() -> Option<Vec<u32>> {
    let res = Command::new("transmission-remote")
        .arg("-l")
        .output()
        .ok()?;
    let data = from_utf8(&res.stdout).ok()?;
    let mut lines: Vec<&str> = data.split("\n").collect();
    lines.pop();
    lines.pop();
    lines.remove(0);
    Some(
        lines
            .iter()
            .map(|l| String::from(l.split_whitespace().next().unwrap()))
            .filter(|s| s.parse::<i32>().is_ok())
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
    )
}

fn try_parse<'a>(re: &Regex, line: &'a str) -> Option<&'a str> {
    let captures = re.captures(line);
    if captures.is_some() {
        return Some(captures.unwrap().get(1).map_or("", |m| m.as_str()).trim());
    }
    return None;
}

pub fn parse_torrent(id: &u32) -> Torrent {
    let id_re = Regex::new(r"Id:(.*)").unwrap();
    let name_re = Regex::new(r"Name:(.*)").unwrap();
    let state_re = Regex::new(r"State:(.*)").unwrap();
    let eta_re = Regex::new(r"ETA:(.*)\(").unwrap();
    let percent_re = Regex::new(r"Percent Done:(.*)").unwrap();
    let download_speed_re = Regex::new(r"Download Speed:(.*)").unwrap();
    let upload_speed_re = Regex::new(r"Upload Speed:(.*)").unwrap();
    let size_re = Regex::new(r"Total size:(.*)\(").unwrap();
    let downloaded_re = Regex::new(r"Have:(.*)\(").unwrap();

    let res = Command::new("transmission-remote")
        .arg(format!("-t{}", id))
        .arg("-i")
        .output()
        .unwrap();
    let data = from_utf8(&res.stdout).unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    let mut id = "?";
    let mut name = "Unknown torrent";
    let mut state = "Unknown";
    let mut eta = "Unknown";
    let mut percent = "0%";
    let mut download_speed = "0 kB/s";
    let mut upload_speed = "0 kB/s";
    let mut size = "0 kB";
    let mut downloaded = "0 kB";

    for line in lines {
        if let Some(s) = try_parse(&id_re, line) {
            id = s;
            continue;
        }
        if let Some(s) = try_parse(&name_re, line) {
            name = s;
            continue;
        }
        if let Some(s) = try_parse(&state_re, line) {
            state = s;
            continue;
        }
        if let Some(s) = try_parse(&eta_re, line) {
            eta = s;
            continue;
        }
        if let Some(s) = try_parse(&percent_re, line) {
            percent = s;
            continue;
        }
        if let Some(s) = try_parse(&download_speed_re, line) {
            download_speed = s;
            continue;
        }
        if let Some(s) = try_parse(&upload_speed_re, line) {
            upload_speed = s;
            continue;
        }
        if let Some(s) = try_parse(&size_re, line) {
            size = s;
            continue;
        }
        if let Some(s) = try_parse(&downloaded_re, line) {
            downloaded = s;
            continue;
        }
    }

    Torrent {
        id: String::from(id),
        name: String::from(name),
        state: String::from(state),
        eta: String::from(eta),
        percent: String::from(percent),
        download_speed: String::from(download_speed),
        upload_speed: String::from(upload_speed),
        size: String::from(size),
        downloaded: String::from(downloaded),
    }
}
