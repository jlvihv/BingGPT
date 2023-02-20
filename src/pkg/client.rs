use super::core::chathub::ChatHub;
use crate::pkg::core::tools::get_path;
use anyhow::{bail, Result};
use colored::Colorize;
use rustyline::{Cmd, Editor, KeyCode, KeyEvent, Modifiers};
use std::io::{stdout, Write};

const INPUT_HISTORY_PATH: &str = "~/.config/binggpt/input-history.txt";
const CONFIG_DIR: &str = "~/.config/binggpt";

pub struct Client {
    chat_hub: ChatHub,
}

impl Client {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        Self::init_config_dir()?;
        let chat_hub = ChatHub::new(cookie_path).await?;
        Ok(Self { chat_hub })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let input = self.input().await?;
            if let Err(e) = self.ask(&input).await {
                println!("send message failed: {}", e.to_string().red());
                println!("You can use `:reset` to reset the conversation.");
            };
            if let Err(e) = self.get_answer().await {
                println!("get answer failed: {}", e.to_string().red());
                println!("You can use `:reset` to reset the conversation.");
            };
        }
    }

    pub async fn ask(&mut self, msg: &str) -> Result<()> {
        self.chat_hub.send_msg(msg).await
    }

    pub async fn get_answer(&mut self) -> Result<()> {
        println!("{}", "Bing:".blue());
        let mut index = 0;
        loop {
            let suggesteds = match self.chat_hub.recv_suggesteds() {
                Ok(suggesteds) => suggesteds,
                Err(e) => {
                    bail!(e)
                }
            };

            if let Some(suggesteds) = suggesteds {
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

    pub async fn input(&mut self) -> Result<String> {
        println!("{}", "You:".cyan());

        let mut rl = Editor::<()>::new().unwrap();
        let _ = rl.load_history(&get_path(INPUT_HISTORY_PATH).unwrap_or_default());
        rl.bind_sequence(KeyEvent(KeyCode::Enter, Modifiers::CTRL), Cmd::Newline);

        let mut input = String::new();
        let mut multi_line_mode: bool = false;
        loop {
            loop {
                let readline = rl.readline("");
                let line = match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        line
                    }
                    // ctrl + c
                    Err(rustyline::error::ReadlineError::Interrupted) => {
                        self.chat_hub.close().await?;
                        std::process::exit(0);
                    }
                    Err(_) => "".to_string(),
                };

                match line.trim() {
                    "" => break,
                    ":exit" | ":quit" | ":q" => {
                        self.chat_hub.close().await?;
                        std::process::exit(0)
                    }
                    ":help" | ":h" => {
                        println!("twice Enter -> send message");
                        println!(":exit, :quit, :q -> exit");
                        println!(":reset -> reset conversation");
                        println!(":help, :h -> show help");
                        println!(":more -> enter multi-line mode");
                        println!(":end -> exit multi-line mode");
                        break;
                    }
                    ":reset" => {
                        if (self.chat_hub.reset().await).is_ok() {
                            println!("{}", "Reset conversation success".green());
                            println!("{}", "You:".cyan());
                        } else {
                            println!("{}", "Reset conversation failed".red());
                            println!("{}", "You:".cyan());
                        };
                        break;
                    }
                    ":more" => {
                        println!(
                            "{}",
                            "You've entered multi-line mode. Enter ':end' to exit multi-line mode"
                                .green()
                        );
                        multi_line_mode = true;
                        break;
                    }
                    ":end" => {
                        multi_line_mode = false;
                        println!();
                        break;
                    }
                    _ => {}
                }
                input.push_str(&line);
            }
            let input = input.trim();
            if input.is_empty() || multi_line_mode {
                continue;
            } else {
                break;
            }
        }
        let _ = rl.append_history(&get_path(INPUT_HISTORY_PATH).unwrap_or_default());
        Ok(input)
    }

    fn init_config_dir() -> Result<()> {
        let config_dir = get_path(CONFIG_DIR)?;
        if !std::path::Path::new(&config_dir).exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(())
    }
}
