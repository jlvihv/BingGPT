<div align="center">
    <img src="https://socialify.git.ci/jlvihv/BingGPT/image?description=1&descriptionEditable=BingGPT%20command%20line%20client%2C%20written%20in%20rust&font=KoHo&language=1&logo=https%3A%2F%2Fupload.wikimedia.org%2Fwikipedia%2Fcommons%2F9%2F9c%2FBing_Fluent_Logo.svg&name=1&owner=1&pattern=Circuit%20Board&theme=Auto" alt="BingGPT" width="640" height="320" />

# BingGPT

_BingGPT command line client, written in rust_

<img src="binggpt.png" />

---

</div>

> This repository hasn't been updated for many days, and may not be updated in the future. It is recommended that you switch to [Aichat](https://github.com/sigoden/aichat).

[中文说明](README_CN.md)

This project is a rust language implementation of [EdgeGPT](https://github.com/acheong08/EdgeGPT), all the hard stuff was done by the original project author `acheong08`, I just wrote it in rust, all credit goes to him, thanks for the hard work big guy!

## Install

```bash
cargo install binggpt-cli
```

```bash
# use `binggpt`
binggpt
```

Or download the binary from the [release page](https://github.com/jlvihv/BingGPT/releases).

## Requirements

You must have a Microsoft account with access to BingGPT.

## Configuration (required)

- for [Chrome](https://chrome.google.com/webstore/detail/cookie-editor/hlkenndednhfkekhgcdicdfddnkalmdm) or [Firefox](https://addons.mozilla.org/en-US/firefox/addon/cookie-editor/) to install the `cookie-editor` extension
- Go to [bing.com](https://www.bing.com) and log in to your Microsoft account
- Open the extension
- Click "Export" in the bottom right corner (this will save your cookies to the clipboard)
- Create or write your cookies to the `~/.config/bing-cookies.json` file

## Usage

> First you need to perform the configuration steps above.

If you have a rust development environment, first you need to clone the code, go to this project directory, and run `cargo run`.

If you want to compile it into binaries, you can run `cargo build --release`. After the compilation is done, you can find the compiled binaries in the `target/release` directory.

If you want to install it to the system, you can run `cargo install --path binggpt-cli`, so you can easily use the `binggpt` command anywhere.

After starting the program, when you see `You:`, it means you can start a conversation with BingGPT, press enter twice to send a message.

In the conversation, you can use the following command.

- `:q` `:quit` `:exit` to quit the program
- `:more` to enter multi-line mode, where you can safely type more text, or paste text from the clipboard
- `:end` exit multi-line mode

## Possible problems

### For Windows10 users

see [#3](https://github.com/jlvihv/BingGPT/issues/3)

Ensure that users running Windows 10 use this command in their terminal, with administrator privileges, to enable text colors in the terminal.

```powershell
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
```

## As a rust crate

```bash
cargo add binggpt
cargo add utf8-slice
cargo add tokio --features full
```

```rust
use std::io::{stdout, Write};

#[tokio::main]
async fn main() {
    let mut bing = binggpt::Bing::new("~/.config/bing-cookies.json")
        .await
        .unwrap();

    // send message
    bing.send_msg("hello").await.unwrap();

    // receive message
    let mut index = 0;

    // loop until the chat is done
    loop {
        if bing.is_done() {
            break;
        }

        let Some(answer) = bing.recv_text().await.unwrap() else{
            continue;
        };

        // print the new part of the answer
        if !answer.is_empty() {
            print!("{}", utf8_slice::from(&answer, index));
            if stdout().flush().is_err() {
                println!("Warning: Failed to flush stdout");
            };
            index = utf8_slice::len(&answer);
        }
    }
}
```

## Contributors

This project exists thanks to all the people who contribute.

 <a href="https://github.com/jlvihv/BingGPT/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=jlvihv/BingGPT" />
 </a>

## License

[MIT](LICENSE)
