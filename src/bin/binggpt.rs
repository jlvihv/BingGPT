use binggpt::pkg::client::Client;
use colored::Colorize;

#[tokio::main]
async fn main() {
    let cookie_path = "~/.config/bing-cookies.json";
    let mut client = match Client::new(cookie_path).await {
        Ok(chat_hub) => chat_hub,
        Err(err) => {
            println!(
                "BingGPT create conversation error: {}",
                err.to_string().red()
            );
            std::process::exit(1);
        }
    };

    if let Err(e) = client.run().await {
        println!("BingGPT run error: {}", e.to_string().red());
        std::process::exit(1);
    };
}
