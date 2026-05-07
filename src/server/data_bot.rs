use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};

use crate::server::fetch::fetch_user::{self, fetch_client_name};

fn get_token() -> &'static str {
    "MTUwMTYxNjUzNzQ3ODYyNzQwOA.GTSVrb.zgRDnPJDq-HiY3f1SN60R47R7QxojjmWRwCN6I"
}

async fn send_msg(ctx: &Context, channel_id: u64, content: &str) {
    let channel = ChannelId::new(channel_id);

    let _ = channel.say(&ctx.http, content).await;
}

pub async fn init() {
    use serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    };

    struct Handler;

    const LOG_CHANNEL: u64 = 1501636741772349601; // channel in bot database, logging

    #[async_trait]
    impl EventHandler for Handler {
        async fn ready(&self, ctx: Context, ready: Ready) {
            println!("{} is online", ready.user.name);
            let msg = format!("Client {} Active", fetch_user::fetch_client_name());
            send_msg(&ctx, LOG_CHANNEL, &msg).await;
        }

        async fn message(&self, ctx: Context, msg: Message) {
            if msg.author.bot {
                return;
            }

            let parts: Vec<&str> = msg.content.split_whitespace().collect();

            if parts.get(0) == Some(&"fetch") {
                if let Some(arg) = parts.get(1) {
                    let text = format!("Attempting to fetch user by id {}", arg);

                    let _ = msg.channel_id.say(&ctx.http, &text).await;
                    let _ = msg.channel_id.say(&ctx.http, fetch_user::fetch_user(arg)).await;
                }
            }
        }
    }

    let token = get_token();

    let intents =
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}