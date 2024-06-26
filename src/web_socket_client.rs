use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};
use tokio_tungstenite::{
    connect_async, tungstenite::http::Uri, tungstenite::protocol::Message, MaybeTlsStream,
    WebSocketStream,
};
use url::Url;

pub struct WebSocketClient {
    server_url: Url,
    incoming_tx: mpsc::Sender<Message>,
    outgoing_rx: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl WebSocketClient {
    pub fn new(server_url: Url) -> (Self, mpsc::Receiver<Message>, mpsc::Sender<Message>) {
        let (incoming_tx, incoming_rx) = mpsc::channel(100);
        let (outgoing_tx, outgoing_rx) = mpsc::channel(100);

        (
            Self {
                server_url,
                incoming_tx,
                outgoing_rx: Arc::new(Mutex::new(outgoing_rx)),
            },
            incoming_rx,
            outgoing_tx,
        )
    }

    pub async fn connect_and_run(&self) {
        loop {
            match self.server_url.to_string().parse::<Uri>() {
                Ok(request_uri) => {
                    match connect_async(request_uri).await {
                        Ok((ws_stream, _)) => {
                            println!("Connected to: {}", self.server_url);
                            self.handle_connection(ws_stream).await; // Correct type expected here
                            break; // Exit loop if connection and handling are successful
                        }
                        Err(e) => {
                            eprintln!("Failed to connect to WebSocket server: {}", e);
                            time::sleep(Duration::from_secs(2)).await; // Wait before retrying
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse URL into a URI: {}", e);
                    time::sleep(Duration::from_secs(2)).await; // Wait before retrying
                }
            }
        }
    }

    async fn handle_connection(&self, ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) {
        let (mut write, mut read) = ws_stream.split();

        // Task to read messages and forward them
        let incoming_tx = self.incoming_tx.clone();
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                if let Ok(msg) = message {
                    if incoming_tx.send(msg).await.is_err() {
                        eprintln!("Error sending message to the channel");
                        break;
                    }
                }
            }
        });

        // Task to send messages from the outgoing queue
        let outgoing_rx = self.outgoing_rx.clone();
        tokio::spawn(async move {
            while let Some(message) = outgoing_rx.lock().await.recv().await {
                if write.send(message).await.is_err() {
                    eprintln!("Failed to send message to WebSocket");
                    break;
                }
            }
        });
    }
}
