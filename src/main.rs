use pkg::bing::ChatHub;

pub mod pkg;

#[tokio::main]
async fn main() {
    let mut chat_hub = ChatHub::new().await.unwrap();
    chat_hub.create_websocket().await.unwrap();
    chat_hub.send_protocol().await.unwrap();
    chat_hub.run().await.unwrap();
}
