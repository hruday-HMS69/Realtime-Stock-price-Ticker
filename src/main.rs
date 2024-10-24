use crate::websocket::start_ticker;

mod websocket;
mod price_feed;

//#[tokio::main]
//async fn main() {
  //  let symbol = "btcusdt"; // Replace with desired trading pair, e.g., ethusdt for Ethereum
    //websocket::start_ticker(symbol).await;
//}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    start_ticker("btcusdt").await;
}

