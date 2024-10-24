use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TickerData {
    pub symbol: String,   // Symbol (e.g., BTCUSDT)
    pub current_price: String,   // Current price (e.g., 34000.50)
}

// Function to process and display ticker data
pub fn process_ticker_data(symbol: &str, price: &str) {
    println!("Symbol: {}, Current Price: ${}", symbol, price);
}
