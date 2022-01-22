use reqwest::header::USER_AGENT;
use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize)]
pub struct VpnConnectionResponse {
    is_connected: bool,
}

pub async fn check_connected_to_vpn(vpn_ip: &str) -> Option<bool> {
    let res = reqwest::Client::new()
        .get("http://ifconfig.io")
        .header(USER_AGENT, "curl/7.77.0") // So that only the IP is returned
        .send()
        .await
        .ok()?;

    let ip = res.text().await.ok()?;

    Some(ip.trim() == vpn_ip)
}

#[get("/")]
pub async fn check_vpn(config: &State<Config>) -> Option<Json<VpnConnectionResponse>> {
    let response = VpnConnectionResponse {
        is_connected: check_connected_to_vpn(&config.vpn_ip).await?,
    };
    Some(Json(response))
}
