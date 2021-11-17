use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.channel_id.eq(&879561100260151356) {
            if msg.content.to_uppercase() == "JED" {
                let _ = msg.channel_id.say(&ctx.http, "what ".to_owned() + msg.author.name.as_str()).await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
    let mut client = Client::builder("ENTER TOKEN HERE")
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}