use futures; // 0.3.4
use reqwest::{Client, Response}; // 0.10.1
use tokio; // 0.2.11

#[tokio::main]
async fn main() {
    let client = Client::new();
    //let urls = vec!["https://api.ipify.org"; 200];
    let urls = vec!["https://bytesonly.com"; 100];
    let bodies = futures::future::join_all(urls.into_iter().map(|url| get(url, &client))).await;

    for b in bodies {
        match b {
            Ok(response) => println!("status is {}", response.status()),
            _ => println!("got an error")
        }
    }
}

async fn get(url: &str, client: &Client) -> Result<Response, reqwest::Error> {
    let a = client.get(url).send().await;
    return a;
}
