use pkg::bing::ChatHub;

pub mod pkg;

#[tokio::main]
async fn main() {
    let mut chat_hub = match ChatHub::new().await {
        Ok(chat_hub) => chat_hub,
        Err(err) => {
            println!("BingGPT create conversation error: {}", err);
            return;
        }
    };

    if let Err(e) = chat_hub.create_websocket().await {
        println!("BingGPT create websocket error: {}", e);
        return;
    };

    if let Err(e) = chat_hub.send_protocol().await {
        println!("BingGPT send protocol error: {}", e);
        return;
    };

    if let Err(e) = chat_hub.run().await {
        println!("BingGPT run error: {}", e);
    };
}
