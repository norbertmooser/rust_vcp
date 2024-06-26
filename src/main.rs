mod web_socket_client;
mod vcp_config;
mod dispatch;

use web_socket_client::WebSocketClient;
use vcp_config::Config;

use tokio::fs;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    // Read the configuration from file
    let config_data = fs::read_to_string("vcp_config.json")
        .await
        .expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&config_data).expect("Failed to parse config.json");

    let (client, incoming_rx, outgoing_tx) = WebSocketClient::new(config.server_url);

    // Start the WebSocket client connection
    tokio::spawn(async move {
        client.connect_and_run().await;
    });

    // Dispatch incoming and outgoing messages
    tokio::spawn(async move {
        dispatch::run_dispatch(incoming_rx, outgoing_tx).await;
    });

    // Async counter task
    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            println!("Counter: {}", counter);
            counter += 1;
            sleep(Duration::from_secs(1)).await;
        }
    });

    // Use tokio::signal or similar to handle graceful shutdown
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
    println!("Shutdown requested!");
}
