use std::process::Command;

#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate reqwest;

mod config;
mod routes;
mod torrent;

fn check_transmission_installed() -> bool {
    Command::new("transmission-daemon").output().is_ok()
}

#[launch]
fn rocket() -> _ {
    if !check_transmission_installed() {
        println!("Failed to launch transmission-daemon");
        std::process::exit(1);
    }
    let config = match config::parse_config() {
        Some(config) => config,
        None => {
            println!("Failed to decode config");
            std::process::exit(1);
        }
    };

    rocket::build()
        .manage(config)
        .mount(
            "/torrents",
            routes![
                routes::torrent::get_torrents,
                routes::torrent::add_torrent,
                routes::torrent::stop_torrent,
                routes::torrent::stop_all_torrents,
                routes::torrent::start_torrent,
                routes::torrent::start_all_torrents,
                routes::torrent::move_torrent,
            ],
        )
        .mount("/vpn", routes![routes::vpn::check_vpn])
}
