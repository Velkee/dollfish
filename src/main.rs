use reqwest::header::{HeaderMap, AUTHORIZATION};

fn main() {
    let mut headers = HeaderMap::new(); 
    headers.insert(AUTHORIZATION, "Client='Dollfish'".parse().unwrap());

    println!("{:#?}", headers)
}