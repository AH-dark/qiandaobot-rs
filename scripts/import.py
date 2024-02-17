#!/usr/bin/python3

import argparse
import asyncio
import os

import psycopg2
from telethon import TelegramClient
from telethon.tl.functions.messages import GetHistoryRequest

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
        posts = await client(GetHistoryRequest(
            peer=channel_entity,
            limit=100,
            offset_date=None,
            offset_id=0,
            max_id=0,
            min_id=0,
            add_offset=0,
            hash=0
        ))
        
        for message in posts.messages:
            # 将消息保存到数据库，考虑到message_id和chat_id共同组成唯一数据
            c.execute(
                "INSERT INTO messages (chat_id, message_id) "
                "VALUES (%s, %s) ON CONFLICT (chat_id, message_id) DO NOTHING",
                (channel_entity.id, message.id))

        conn.commit()


# 运行主函数
async def main():
    await client.start()
    await save_messages_to_db(args.channel)

    # 关闭数据库连接
    conn.close()


if __name__ == '__main__':
    asyncio.run(main())
