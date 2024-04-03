use std::{collections::HashMap, env};

use reqwest::header::{HeaderMap, AUTHORIZATION};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!(
            "MediaBrowser Client=\"Dollfish Jellyfin Client\", Device=\"{}\", DeviceId=\"test\", Version=\"0.1.0\", Token=\"\"",
            env::consts::OS,
        )
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
