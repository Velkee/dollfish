use reqwest::header::{HeaderMap, AUTHORIZATION};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, "Client='Dollfish'".parse().unwrap());
    headers.append(AUTHORIZATION, "Device='hepahestus'".parse().unwrap());
    headers.append(AUTHORIZATION, "DeviceId='testdevice'".parse().unwrap());
    headers.append(AUTHORIZATION, "Version='0.1.0'".parse().unwrap());

    let authentication_results = client
        .post("http://localhost:8096/Users/AuthenticateByName")
        // .headers(headers)
        .body("Test")
        .send()
        .await
        .unwrap()
        .status();

    println!("{}", authentication_results)
}
