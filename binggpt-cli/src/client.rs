// code needs sqlite support

use anyhow::{bail, Result};
use binggpt::tools::get_path;
use binggpt::Bing;
use colored::Colorize;
use std::io::{stdout, Write};
use rusqlite::{params, Connection};

use crate::user_input;

const CONFIG_DIR: &str = "~/.config/binggpt";
const DB_PATH: &str = "~/.config/binggpt/chat.db";

pub struct Client {
    bing: Bing,
    db_conn: Connection,
}

impl Client {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        Self::init_config_dir()?;
        let bing = Bing::new(cookie_path).await?;
        let db_conn = Connection::open(get_path(DB_PATH)?)?;
        db_conn.execute(
            "CREATE TABLE IF NOT EXISTS chat (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                input TEXT NOT NULL,
                output TEXT NOT NULL
             )",
            params![],
        )?;
        Ok(Self { bing, db_conn })
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
            self.save_chat(&input, "")?;
        }
    }

    pub async fn ask(&mut self, msg: &str) -> Result<()> {
        self.bing.send_msg(msg).await
    }

    pub async fn get_answer(&mut self) -> Result<()> {
        println!("{}", "Bing:".blue());
        let mut index = 0;
        loop {
            if self.bing.is_done() {
                let suggesteds = match self.bing.recv_suggesteds() {
                    Ok(suggesteds) => suggesteds,
                    Err(e) => {
                        bail!(e)
                    }
                };

                if let Some(suggesteds) = suggesteds {
                    if suggesteds.is_empty() {
                        println!("  {}", "No suggesteds".yellow());
                        println!("  {}", "You may have reached the maximum number of chats. The limit is 5 times.".yellow());
                        println!(
                            "  {}",
                            "You can use `:reset` to reset the conversation.".yellow()
                        );
                    }
                    println!("\n{}", "Suggesteds:".purple());
                    for suggested in suggesteds {
                        println!("  {}", suggested);
                    }
                    println!();
                };

                break;
            }

            let Some(answer) = self.bing.recv_text().await? else {
                continue;
            };
            if !answer.is_empty() {
                print!("{}", utf8_slice::from(&answer, index));
                if stdout().flush().is_err() {
                    println!("{}", "Warning: Failed to flush stdout".yellow());
                };
                index = utf8_slice::len(&answer);
                self.save_chat("", &answer)?;
            }
        }
        Ok(())
    }

    pub async fn input(&mut self) -> Result<String> {
        loop {
            println!("{}", "You:".cyan());
            let user_input = user_input::input();
            match user_input {
                user_input::Input::Text(input) => {
                    self.save_chat(&input, "")?;
                    return Ok(input);
                }
                user_input::Input::Command(cmd) => match cmd {
                    user_input::Command::Exit => {
                        println!("Bye!");
                        std::process::exit(0);
                    }
                    user_input::Command::Help => {
                        println!(":exit, :quit, :q: exit the program");
                        println!(":help, :h: show this help");
                        println!(":reset: reset the conversation");
                        println!(":more: enter multi-line mode(in linux and macos)");
                        println!(":end: exit multi-line mode(in linux and macos)");
                    }
                    user_input::Command::Reset => {
                        self.bing.reset().await?;
                        self.save_chat("", "")?;
                        println!("Reset the conversation.");
                    }
                    _ => {}
                },
            }
        }
    }

    fn init_config_dir() -> Result<()> {
        let config_dir = get_path(CONFIG_DIR)?;
        if !std::path::Path::new(&config_dir).exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(())
    }

    fn save_chat(&self, input: &str, output: &str) -> Result<()> {
        self.db_conn.execute(
            "INSERT INTO chat (input, output) VALUES (?, ?)",
            params![input, output],
        )?;
        Ok(())
    }
}
