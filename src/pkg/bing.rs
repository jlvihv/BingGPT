use std::io::{stdout, Write};

use super::core::chathub::ChatHub;
use anyhow::Result;
use colored::Colorize;

pub struct Bing {
    chat_hub: ChatHub,
}

impl Bing {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        let chat_hub = ChatHub::new(cookie_path).await?;
        Ok(Self { chat_hub })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let input = self.input();
            self.ask(&input).await?;
            self.get_answer().await?;
        }
    }

    pub async fn ask(&mut self, msg: &str) -> Result<()> {
        self.chat_hub.send_msg(msg).await
    }

    pub async fn get_answer(&mut self) -> Result<()> {
        println!("{}", "Bing:".blue());
        let mut index = 0;
        loop {
            if let Some(suggesteds) = self.chat_hub.recv_suggesteds() {
                println!("\n{}", "Suggesteds:".purple());
                for suggested in suggesteds {
                    println!("  {}", suggested);
                }
                println!();
                break;
            };

            let Some(answer) = self.chat_hub.recv_text().await? else {
                continue;
            };
            if !answer.is_empty() {
                print!("{}", utf8_slice::from(&answer, index));
                if stdout().flush().is_err() {
                    println!("{}", "Warning: Failed to flush stdout".yellow());
                };
                index = utf8_slice::len(&answer);
            }
        }
        Ok(())
    }

    pub fn input(&self) -> String {
        println!("{}", "You:".cyan());
        let mut input = String::new();
        let mut more_line_mode = false;
        loop {
            loop {
                let mut line = String::new();
                if std::io::stdin().read_line(&mut line).is_err() {
                    println!(
                        "{}",
                        "Warning: Failed to read line, this line is not invalid, please re-enter"
                            .yellow()
                    );
                    println!("\n{}", "You:".cyan());
                    continue;
                };
                match line.trim() {
                    "" => break,
                    ":more" => {
                        println!("{}", "You've entered multi-line mode. Enter ':end' to end the multi-line mode".green());
                        more_line_mode = true;
                        break;
                    }
                    ":end" => {
                        more_line_mode = false;
                        println!();
                        break;
                    }
                    ":exit" | ":quit" | ":q" => std::process::exit(0),
                    _ => {}
                }
                input.push_str(&line)
            }
            let input = input.trim().to_string();
            if input.is_empty() || more_line_mode {
                continue;
            } else {
                break;
            }
        }
        input
    }
}
