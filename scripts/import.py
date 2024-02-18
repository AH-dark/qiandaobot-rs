#!/usr/bin/python3

import argparse
import asyncio
import os

import psycopg2
from telethon import TelegramClient

# 使用 argparse 获取命令行参数
parser = argparse.ArgumentParser(description='Save Telegram messages to PostgreSQL database.')
parser.add_argument('--api_id', help='Your Telegram API ID', required=True)
parser.add_argument('--api_hash', help='Your Telegram API Hash', required=True)
parser.add_argument('--channel', help='Telegram channel username', required=True)

args = parser.parse_args()

db_connection = os.getenv('DATABASE_URL')

client = TelegramClient('session_name', args.api_id, args.api_hash)

conn = psycopg2.connect(db_connection)
c = conn.cursor()


# 异步获取消息并保存到数据库的函数
async def save_messages_to_db(channel_username):
    async with client:
        channel_entity = await client.get_entity(channel_username)
        async for message in client.iter_messages(channel_entity):
            if message.action is None:
                print(f"Saving message {message.id} from {channel_entity.id} to database.")

                # 将消息保存到数据库，考虑到message_id和chat_id共同组成唯一数据
                c.execute(
                    "INSERT INTO messages (chat_id, message_id) "
                    "VALUES (%s, %s) ON CONFLICT (chat_id, message_id) DO NOTHING",
                    (message.chat_id, message.id))
            else:
                print(f"Skipping message {message.id} from {channel_entity.id}.")

        conn.commit()


# 运行主函数
async def main():
    client.start()
    await save_messages_to_db(args.channel)

    # 关闭数据库连接
    conn.close()


if __name__ == '__main__':
    asyncio.run(main())
