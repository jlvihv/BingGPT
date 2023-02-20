use colored::Colorize;
use rustyline::Editor;
use std::str::FromStr;

pub enum Input {
    Text(String),
    Command(Command),
}

pub enum Command {
    Exit,
    Help,
    Reset,
    More,
    End,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            ":exit" | ":quit" | ":q" => Ok(Input::Command(Command::Exit)),
            ":help" | ":h" => Ok(Input::Command(Command::Help)),
            ":reset" => Ok(Input::Command(Command::Reset)),
            ":more" => Ok(Input::Command(Command::More)),
            ":end" => Ok(Input::Command(Command::End)),
            _ => Ok(Input::Text(s.to_string())),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn input() -> Input {
    use rustyline::{Cmd, KeyCode, KeyEvent, Modifiers};

    let mut rl = Editor::<()>::new().unwrap();
    rl.bind_sequence(KeyEvent(KeyCode::Enter, Modifiers::SHIFT), Cmd::Newline);

    let readline = rl.readline("");
    let line = match readline {
        Ok(line) => line,
        // ctrl + c
        Err(rustyline::error::ReadlineError::Interrupted) => {
            return Input::Command(Command::Exit);
        }
        Err(_) => "".to_string(),
    };

    Input::from_str(&line).unwrap()
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn input() -> Input {
    let mut rl = Editor::<()>::new().unwrap();

    let mut input = String::new();
    let mut multi_line_mode = false;
    loop {
        loop {
            let readline = rl.readline("");
            let line = match readline {
                Ok(line) => line,
                // ctrl + c
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    return Input::Command(Command::Exit);
                }
                Err(_) => "".to_string(),
            };

            let line = Input::from_str(&line).unwrap();

            match line {
                Input::Command(Command::More) => {
                    println!(
                        "{}",
                        "You've entered multi-line mode. Enter ':end' to exit multi-line mode"
                            .green()
                    );
                    multi_line_mode = true;
                    break;
                }
                Input::Command(Command::End) => {
                    multi_line_mode = false;
                    break;
                }
                Input::Text(s) => {
                    if s.is_empty() {
                        break;
                    }
                    input.push_str(&s);
                }
                _ => {
                    return line;
                }
            }
        }
        if input.trim().is_empty() || multi_line_mode {
            continue;
        } else {
            break;
        }
    }
    Input::Text(input)
}
