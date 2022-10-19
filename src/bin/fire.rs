use chrono::Local;
use futures::future::join_all;
use hyper::Client;
use log::info;
// use tokio::time::{sleep, Duration};

const PROXY_URL: &str = "http://127.0.0.1:3000/";
const NUM_REQUESTS: u32 = 50;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    // This is where we will setup our HTTP client requests.
    let client = Client::new();
    let mut join_handles = vec![];

    info!("Starting requests...");
    let start_time = Local::now();
    for request_num in 1..(NUM_REQUESTS + 1) {
        let url = format!("{PROXY_URL}{}", request_num).parse().unwrap();
        join_handles.push(async {
            let resp = client.get(url).await?;
            hyper::body::to_bytes(resp.into_body()).await
        });
        // sleep(Duration::from_millis(5)).await;
    }
    info!(
        "Began sending {NUM_REQUESTS} requests in {}us",
        (Local::now() - start_time).num_microseconds().unwrap()
    );

    // Wait on both them at the same time:
    // let res = futures::try_join!(join_handles..)?;
    join_all(join_handles).await;

    info!(
        "All {} requests returned after {}ms",
        NUM_REQUESTS,
        (Local::now() - start_time).num_milliseconds()
    );

    Ok(())
}
