use reqwest::header::{HeaderMap, AUTHORIZATION};

use crate::VERSION;

pub async fn set_auth_header(
    mut header_map: HeaderMap,
    device: String,
    device_id: String,
    token: Option<String>,
) -> HeaderMap {
    let content = match token {
        Some(token) => format!("MediaBrowser Client=\"Dollfish Jellyfin Client\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\", Token=\"{}\"", device, device_id, VERSION, token),
        None => format!("MediaBrowser Client=\"Dollfish Jellyfin Client\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\", Token=\"\"", device, device_id, VERSION),
    };

    header_map.insert(AUTHORIZATION, content.parse().unwrap());

    header_map
}
