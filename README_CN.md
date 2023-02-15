# BingGPT

BingGPT 命令行客户端，使用 rust 编写。

这个项目是 [EdgeGPT](https://github.com/acheong08/EdgeGPT) 的 rust 语言实现，所有困难的事情都是原项目作者 `acheong08` 完成的，我仅仅是用 rust 写了一遍，所有的功劳都归功于他，感谢大佬的辛勤付出！

## 要求

你必须有一个可以访问 BingGPT 的微软账户。

## 配置

你需要打开 `bing.com` 并登录，然后在浏览器的开发者工具中找到 `Application` 选项卡，然后找到 `Cookies`，找到 `bing.com`，然后找到 `_U` 字段和 `KievRPSSecAuth`字段。

将它们的值填写到 `~/.config/bing-cookies.toml` 中，格式如下：

```toml
u="_U字段"
kiev="KievRPSSecAuth字段"
```

## 使用方法

> 首先你需要执行上面的配置步骤。
>
> 如果你现在没有可以访问 BingGPT 的微软账户，你可以将本项目下的 `bing-cookies.toml` 文件复制到 `~/.config/` 目录下，这是一个暂时可用的 cookies，但是随时可能失效，不要依赖它，它只是为了方便测试。如果它失效，我也不会更新它。

如果你有 rust 开发环境，首先你需要克隆代码，进入本项目目录，然后运行 `cargo run` 即可。

## 工作正在进行中
