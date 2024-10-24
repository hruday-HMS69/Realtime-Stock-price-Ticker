use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use crate::price_feed::process_ticker_data;
use futures_util::StreamExt;
use std::time::Instant; // Import to measure time

#[derive(serde::Deserialize, Debug)]
struct TickerData {
    s: String,   // Symbol
    c: String,   // Current price
}

pub async fn start_ticker(symbol: &str) {
    // WebSocket URL for Binance's ticker updates
    let binance_ws_url = format!("wss://stream.binance.com:9443/ws/{}@ticker", symbol);

    // Connect to Binance WebSocket
    let (ws_stream, _) = connect_async(Url::parse(&binance_ws_url).unwrap())
        .await
        .expect("Failed to connect to Binance WebSocket");

    println!("Connected to Binance WebSocket for {}", symbol);

    let (mut _write, mut read) = ws_stream.split();

    // Listen for incoming messages
    while let Some(message) = read.next().await {
        let receive_time = Instant::now(); // Timestamp when message is received

        match message {
            Ok(Message::Text(text)) => {
                let process_time = Instant::now(); // Timestamp when processing starts
                if let Ok(ticker_data) = serde_json::from_str::<TickerData>(&text) {
                    // Measure latency
                    let latency = process_time.duration_since(receive_time).as_micros(); // in microseconds
                    println!("Latency: {} µs", latency);

                    process_ticker_data(&ticker_data.s, &ticker_data.c);
                }
            }
            Ok(Message::Binary(_)) => {
                println!("Received binary message");
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                println!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }
}
