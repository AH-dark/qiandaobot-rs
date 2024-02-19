# 签到 Bot

签到 Bot 提供了向某些烧鸡签到的方法。你可以创建一个频道用于存储消息，本 Bot 会监听频道并对用户发起的 `/qiandao` 命令进行回复。本
Bot 灵感来自于~~大烧鸡~~ [乌班](https://t.me/one187) 所创造的基于 Python 的[纯小亦](https://t.me/chunxiaoyi)签到 Bot。

> 这是本人第一个 Rust 项目，如有不足之处，还请多多指教。

## 使用方法

1. 创建一个 Bot，获取其 Token。
2. 创建一个频道，将 Bot 添加到频道中，并将其设置为管理员。
3. 获取频道的 ID。
4. 设置环境变量
5. 运行程序

| 环境变量（标记 * 为必须）    | 用途                           | 示例                                                             |
|-------------------|------------------------------|----------------------------------------------------------------|
| `CHANNEL_ID`*     | 监听的频道 ID                     | `-1001910581529`                                               |
| `DATABASE_URL`*   | 数据库 URL，支持 Postgres 和 Sqlite | `postgresql://qiandaobot:qiandaobot@localhost:5432/qiandaobot` |
| `TELOXIDE_TOKEN`* | Telegram Bot Token           |                                                                |
| `RUST_LOG`        | Log Level                    | info                                                           |

## 编译

```shell
make build # 编译全部 
make build-qiandaobot # 编译签到 Bot 本体
```
