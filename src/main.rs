/*
    Dollfish. A jellyfin to MPV shim program.
    Copyright (C) 2024  Velkee

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{collections::HashMap, env};

use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    uuid: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let hostname = match System::host_name() {
        Some(hostname) => hostname,
        None => {
            println!("WARNING: Hostname could not be determined, falling back to OS.");
            env::consts::OS.to_string()
        }
    };

    // TODO: save the uuid somehow to be reused so it doesn't generate 1000 devices in Jellyfin
    let uuid = Uuid::new_v4();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("MediaBrowser Client=\"Dollfish Jellyfin Client\", Device=\"{hostname}\", DeviceId=\"{uuid}\", Version=\"{VERSION}\", Token=\"\"")
        .parse()
        .unwrap(),
    );

    let test_request = client
        .get("http://localhost:8096/System/Info/Public")
        .send()
        .await;

    match test_request {
        Ok(_) => (),
        Err(_) => {
            println!("Could not contact server, please verify the server is running and try again");
            return Ok(());
        }
    }

    let mut user_credentials = HashMap::new();
    user_credentials.insert("Username", "velkee");
    user_credentials.insert("Pw", "password");

    let authenticate = client
        .post("http://localhost:8096/Users/AuthenticateByName")
        .headers(headers)
        .json(&user_credentials)
        .send()
        .await?;

    let response = authenticate.text().await?;

    println!("Response: {}", response);

    Ok(())
}
