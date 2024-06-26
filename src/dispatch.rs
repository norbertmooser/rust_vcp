use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time;
use tokio_tungstenite::tungstenite::protocol::Message;

// Function to handle incoming messages
pub async fn handle_incoming_messages(mut incoming_rx: Receiver<Message>) {
    while let Some(message) = incoming_rx.recv().await {
        println!("Handling incoming message: {:?}", message);
        // Additional processing can be implemented here
    }
}

// Function to generate and send outgoing messages
pub async fn generate_outgoing_messages(outgoing_tx: Sender<Message>) {
    let mut interval = time::interval(Duration::from_secs(10));
    let mut counter = 0;
    loop {
        interval.tick().await;
        let message = Message::Text(format!("Counter message {}", counter));
        counter += 1;

        if outgoing_tx.send(message).await.is_err() {
            eprintln!("Failed to send outgoing message");
            break;
        }
    }
}

// Function to run both incoming and outgoing message processes
pub async fn run_dispatch(incoming_rx: Receiver<Message>, outgoing_tx: Sender<Message>) {
    let handle_incoming = handle_incoming_messages(incoming_rx);
    let generate_outgoing = generate_outgoing_messages(outgoing_tx);

    // Use `tokio::join!` to run both tasks concurrently.
    tokio::join!(handle_incoming, generate_outgoing);
}
