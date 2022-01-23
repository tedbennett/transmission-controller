use std::{process::Command, str::from_utf8};

use rocket::{response::status, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    torrent::{get_torrent_ids, parse_torrent, Torrent},
};

use super::vpn::check_connected_to_vpn;

#[derive(Serialize)]
pub struct AddTorrentResponse {
    success: bool,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct TorrentListResponse {
    torrents: Vec<Torrent>,
}

#[derive(Deserialize)]
pub struct AddTorrentRequest {
    magnet: String,
}

#[derive(Deserialize)]
pub struct MoveTorrentRequest {
    id: String,
    destination: String,
}

enum DownloadDestination {
    TV,
    Movies,
}

impl DownloadDestination {
    fn path<'a>(&self, config: &'a Config) -> &'a str {
        match self {
            DownloadDestination::TV => &config.tv_path,
            DownloadDestination::Movies => &config.movies_path,
        }
    }

    fn from(str: &str) -> Option<DownloadDestination> {
        match str {
            "tv" => Some(DownloadDestination::TV),
            "movies" => Some(DownloadDestination::Movies),
            _ => None,
        }
    }
}

#[get("/")]
pub fn get_torrents() -> Option<Json<TorrentListResponse>> {
    let ids = get_torrent_ids()?;
    let torrents: Vec<Torrent> = ids.iter().map(|id| parse_torrent(id)).collect();
    Some(Json(TorrentListResponse { torrents }))
}

#[post("/", data = "<data>")]
pub async fn add_torrent(
    config: &State<Config>,
    data: Json<AddTorrentRequest>,
) -> Option<Json<AddTorrentResponse>> {
    if !check_connected_to_vpn(&config.vpn_ip).await? {
        return Some(Json(AddTorrentResponse {
            success: false,
            error: Some(String::from("VPN not connected")),
        }));
    }
    let res = Command::new("transmission-remote")
        .arg("-a")
        .arg(&data.magnet)
        .output()
        .ok()?;

    let status = from_utf8(&res.stdout)
        .ok()?
        .split_whitespace()
        .last()
        .unwrap();
    let response = AddTorrentResponse {
        success: status.trim() == "\"success\"",
        error: None,
    };

    Some(Json(response))
}

#[put("/start")]
pub fn start_all_torrents() -> Option<status::NoContent> {
    let ids = get_torrent_ids()?;
    for id in &ids {
        Command::new("transmission-remote")
            .arg(format!("-t{}", id))
            .arg("-s")
            .output()
            .ok()?;
    }
    Some(status::NoContent)
}

#[put("/start/<id>")]
pub fn start_torrent(id: String) -> Option<status::NoContent> {
    Command::new("transmission-remote")
        .arg(format!("-t{}", id))
        .arg("-s")
        .output()
        .ok()?;
    Some(status::NoContent)
}

#[put("/stop")]
pub fn stop_all_torrents() -> Option<status::NoContent> {
    let ids = get_torrent_ids()?;
    for id in &ids {
        Command::new("transmission-remote")
            .arg(format!("-t{}", id))
            .arg("-S")
            .output()
            .ok()?;
    }
    Some(status::NoContent)
}

#[put("/stop/<id>")]
pub fn stop_torrent(id: String) -> Option<status::NoContent> {
    Command::new("transmission-remote")
        .arg(format!("-t{}", id))
        .arg("-S")
        .output()
        .ok()?;
    Some(status::NoContent)
}

#[put("/move", data = "<data>")]
pub fn move_torrent(
    config: &State<Config>,
    data: Json<MoveTorrentRequest>,
) -> Option<status::NoContent> {
    let destination = DownloadDestination::from(&data.destination)?.path(&config);

    Command::new("transmission-remote")
        .arg(format!("-t{}", &data.id))
        .arg("-S")
        .arg(destination)
        .output()
        .ok()?;

    Command::new("transmission-remote")
        .arg(format!("-t{}", &data.id))
        .arg("--move")
        .arg(destination)
        .output()
        .ok()?;

    Command::new("transmission-remote")
        .arg(format!("-t{}", &data.id))
        .arg("-S")
        .arg(destination)
        .output()
        .ok()?;

    Some(status::NoContent)
}
