use super::core::chathub::ChatHub;
use anyhow::Result;
use colored::Colorize;
use rustyline::Editor;
use std::io::{stdout, Write};

pub struct Client {
    chat_hub: ChatHub,
}

impl Client {
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
        // TODO: Plan to send messages with "Ctrl + Enter"

        println!("{}", "You:".cyan());

        let mut rl = Editor::<()>::new().unwrap();
        let mut input = String::new();
        let mut more_line_mode = false;
        loop {
            loop {
                let readline = rl.readline("");
                let line = match readline {
                    Ok(line) => line,

                    // ctrl + c
                    Err(rustyline::error::ReadlineError::Interrupted) => {
                        std::process::exit(0);
                    }

                    Err(_) => "".to_string(),
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
