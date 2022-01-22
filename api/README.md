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

### Check VPN status

**Definition**

`GET /vpn`

**Response**

- `200 OK` on success

```json
{
  "is_connected": true
}
```

### List of torrents

**Definition**

`GET /torrents`

**Response**

- `200 OK` on success

```json
[
  {
    "id": "1",
    "name": "Cowboy Bebop",
    "state": "Downloading",
    "eta": "3 Hours",
    "percent": "3.5%",
    "download_speed": "10 MB/s",
    "upload_speed": "0 kB/s",
    "size": "10.92 GB",
    "downloaded": "1.27 GB"
  }
]
```

### Add a torrent

**Definition**

`PUT /torrents`

**Arguments**

- `"magnet": string` a valid magnet link

**Response**

- `204 No Content` on success

### Start all torrents

**Definition**

`PUT /torrents/start`

**Response**

- `204 No Content` on success

### Stop all torrents

**Definition**

`PUT /torrents/stop`

**Response**

- `204 No Content` on success

### Start a torrent

**Definition**

`PUT /torrents/start/<id>`

**Response**

- `204 No Content` on success

### Stop a torrent

**Definition**

`PUT /torrents/stop/<id>`

**Response**

- `204 No Content` on success

### Move a torrent destination

**Definition**

`PUT /torrents/move`

**Arguments**

- `"id": string` the torrent id
- `"destination": string` the destination type, at the moment either `tv` or `movies`

**Response**

- `204 No Content` on success
