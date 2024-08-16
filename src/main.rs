use std::collections::HashMap;

use reqwest::header::{HeaderMap, AUTHORIZATION};

mod api;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        "MediaBrowser Client=\"Dollfish\", Device=\"testdevice\", DeviceId=\"testdevice\", Version=\"0.1.0\""
            .parse()
            .unwrap(),
    );

    let mut auth = HashMap::new();

    auth.insert("Username", "velkee");
    auth.insert("Pw", "Password");

    let authentication_results = client
        .post("http://localhost:8096/Users/AuthenticateByName")
        .headers(headers)
        .json(&auth)
        .send()
        .await
        .unwrap();

    println!("{}", authentication_results.status());

    println!("{}", authentication_results.text().await.unwrap())
}
