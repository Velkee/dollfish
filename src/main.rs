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

use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    device_name: String,
    uuid: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let mut config_file = match File::open("./dollfish.conf") {
        Ok(file) => file,
        Err(_) => {
            println!("Config doesn't exist, creating with defaults");

            let hostname = match System::host_name() {
                Some(hostname) => hostname,
                None => {
                    println!("WARNING: Hostname could not be determined, falling back to OS.");
                    env::consts::OS.to_string()
                }
            };
            let uuid = Uuid::new_v4();

            let config = Config {
                device_name: hostname,
                uuid: uuid.to_string(),
            };

            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .truncate(true)
                .open("./dollfish.conf")
                .expect("Could not create config file");

            file.write_all(toml::to_string(&config).unwrap().as_bytes())
                .expect("Could not write to new config");

            file
        }
    };

    let mut config = String::new();
    config_file
        .read_to_string(&mut config)
        .expect("Could not read from config");

    let config: Config =
        toml::from_str(&config).expect("Could not convert config from TOML to Struct");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("MediaBrowser Client=\"Dollfish Jellyfin Client\", Device=\"{}\", DeviceId=\"{}\", Version=\"{}\", Token=\"\"", config.device_name, config.uuid, VERSION)
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
