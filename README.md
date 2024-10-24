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


📦 Project Structure

src/
│
├── main.rs          # Entry point of the application
├── websocket.rs     # WebSocket connection and message handling
├── price_feed.rs    # Processing and displaying price data
├── utils.rs         # Utility functions (if any)
└── Cargo.toml       # Project dependencies and configuration


⚙️ Setup & Installation

Prerequisites
Make sure you have Rust installed. If not, you can install it from here.
Step-by-Step Guide
Clone the repository:

bash
Copy code
git clone https://github.com/yourusername/your-repo-name.git
cd your-repo-name
Install dependencies:

bash
Copy code
cargo build
Run the application:

bash
Copy code
cargo run -- <symbol>
Replace <symbol> with the cryptocurrency symbol (e.g., btcusdt for Bitcoin to USDT).

📈 Usage Example
To see real-time updates and latency measurements for Bitcoin to USDT:

bash
Copy code
cargo run -- btcusdt
Example Output:

makefile
Copy code
Connected to Binance WebSocket for btcusdt
BTC/USDT: $29450.23
Latency: 50 µs
BTC/USDT: $29451.05
Latency: 45 µs


⚡ Optimizations & Future Enhancements

Better error handling: Improve how the application handles WebSocket disconnections and errors.
Support for multiple symbols: Add support to monitor multiple symbols at once.
Visual dashboard: Create a simple UI to visualize prices and latency in real-time.
Further optimizations: Optimize deserialization and data handling for even lower latency.
