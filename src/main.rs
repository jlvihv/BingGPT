use colored::Colorize;
use pkg::bing::Bing;

pub mod pkg;

#[tokio::main]
async fn main() {
    let cookie_path = "~/.config/bing-cookies.json";
    let mut bing = match Bing::new(cookie_path).await {
        Ok(chat_hub) => chat_hub,
        Err(err) => {
            println!(
                "BingGPT create conversation error: {}",
                err.to_string().red()
            );
            return;
        }
    };

    if let Err(e) = bing.run().await {
        println!("BingGPT run error: {}", e.to_string().red());
    };
}
