use loony::http::client::{error::SendRequestError, Client};

#[loony::main]
async fn main() -> Result<(), SendRequestError> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    env_logger::init();

    let client = Client::new();

    // Create request builder, configure request and send
    let mut response = client
        .get("https://www.rust-lang.org/")
        .header("User-Agent", "loony")
        .send()
        .await?;

    // server http response
    println!("Response: {:?}", response);

    // read response body
    let body = response.body().await.unwrap();
    println!("Downloaded: {:?} bytes", body.len());

    Ok(())
}
