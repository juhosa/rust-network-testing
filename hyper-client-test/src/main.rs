use hyper::{body::HttpBody, Client};
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");

    let client = Client::new();

    let uri = "http://httpbin.org/ip".parse()?;

    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
