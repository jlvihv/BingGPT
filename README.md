# BingGPT

[中文版本](README_CN.md)

BingGPT command line client, written in rust.

This project is the rust language implementation of [EdgeGPT](https://github.com/acheong08/EdgeGPT). All the difficult things were done by the original project author `acheong08`. I just wrote it in rust. All the credit goes to him, thank you for your hard work!

## Require

You must have a Microsoft account with access to BingGPT.

## configuration

You need to open `bing.com` and log in, then in the browser's developer tools find the `Application` tab, then find `Cookies`, find `bing.com`, then find the `_U` field and `KievRPSSecAuth` field.

Fill their values into `~/.config/bing-cookies.toml` in the following format:

```toml
u="_U field"
kiev="KievRPSSecAuth field"
```

## Instructions

> First you need to perform the configuration steps above.
>
> If you do not have a Microsoft account that can access BingGPT now, you can copy the `bing-cookies.toml` file under this project to the `~/.config/` directory. This is a temporarily available cookie, but it may be used at any time Fail, don't rely on it, it's just for testing convenience. If it fails, I won't update it either.

If you have a rust development environment, first you need to clone the code, enter the project directory, and then run `cargo run`.

## work in progress
