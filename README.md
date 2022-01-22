# Transmission Controller

This is a server written in Rust using [Rocket](https://rocket.rs/) to control the [transmission BitTorrent client](https://transmissionbt.com/) running locally.

## Setup

Make sure you have transmission-cli installed and running. On Mac, run:

`brew install transmission`

and launch the daemon with:

`transmission-daemon`

At the moment, there is a required config.json file which includes destinations for TV and Movie downloads, as well as your VPN's IP address. This should be kept at the root directory and look like:

```json
{
  "tv": "/path/to/tv",
  "movies": "/path/to/movies",
  "vpn_ip": "192.168.0.1"
}
```

This isn't the best solution for this at the moment and I plan to change it in the future.

Run the project in a dev environment with `cargo run`

## API

### GET /vpn

Returns:

```json
{
    "is_connected": bool
}
```

### GET /torrents

Returns:

```json
{
    "torrents": [Torrent]
}
```

where Torrent is:

```json
{
    "id": string,
    "name": string,
    "state": string,
    "eta": string,
    "percent": string,
    "download_speed": string,
    "upload_speed": string,
    "size": string,
    "downloaded": string,
}
```

### PUT /torrents

Adds a torrent

Body:

```json
{
    "magnet": string
}
```
