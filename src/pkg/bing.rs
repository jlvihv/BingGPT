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
        self.chat_hub.send_msg(msg).await?;
        Ok(())
    }

    pub async fn get_answer(&mut self) -> Result<()> {
        println!("{}", "Bing:".blue());
        let mut index = 0;
        loop {
            let msg = self.chat_hub.recv_msg().await?;
            // println!("{}", msg);
            if gjson::get(&msg, "type").i32() == 1 {
                let answer = gjson::get(&msg, "arguments.0.messages.0.adaptiveCards.0.body.0.text")
                    .to_string();
                if !answer.is_empty() {
                    print!("{}", utf8_slice::from(&answer, index));
                    if stdout().flush().is_err() {
                        println!("{}", "Warning: Failed to flush stdout".yellow());
                    };
                    index = utf8_slice::len(&answer);
                }
            }
            if gjson::get(&msg, "type").i32() == 2 {
                let suggesteds = gjson::get(&msg, "item.messages.1.suggestedResponses.#.text");
                println!("\n{}", "Suggestions:".purple());
                for suggested in suggesteds.array() {
                    println!("  {}", suggested);
                }
                println!();
                break;
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
                    continue;
                };
                if line.trim().is_empty() {
                    break;
                } else if line.trim() == ":more" {
                    println!("{}", "(Enter ':end' to end the multi-line mode.)".green());
                    more_line_mode = true;
                    break;
                } else if line.trim() == ":end" {
                    more_line_mode = false;
                    break;
                }
                input.push_str(&line)
            }
            let input = input.trim().to_string();
            if input.is_empty() {
                continue;
            } else if input == ":exit" || input == ":quit" || input == ":q" {
                std::process::exit(0);
            } else if more_line_mode {
                continue;
            } else {
                break;
            }
        }
        input
    }
}
