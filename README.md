Real-Time Stock Price Ticker with Latency Measurement
📊 Overview
This project is a real-time stock price ticker that connects to the Binance WebSocket API using Rust. It fetches live price updates for a specified cryptocurrency pair (like BTC/USDT) and displays the current price along with the latency of the data received. The project demonstrates low-latency data processing and efficient handling of real-time WebSocket data.

🚀 Features
Real-time stock price updates from Binance.
Measurement of latency between receiving and processing data.
Efficient WebSocket handling using tokio and tokio-tungstenite.
Built in Rust, emphasizing performance and low latency.
🛠️ Technologies Used
Rust: For building a high-performance and low-latency application.
Tokio: For asynchronous programming.
tokio-tungstenite: For WebSocket communication.
serde: For JSON serialization and deserialization.
