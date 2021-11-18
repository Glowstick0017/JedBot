use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // dont read bot messages
        if !msg.author.bot {
            // only responds in #bot-commands
            if msg.channel_id.eq(&879561100260151356) {
                
                if msg.author.id == 417116039789281300 {
                  let _ = msg.reply_ping(&ctx.http, "Screw You ".to_owned() + msg.author.name.as_str()).await;
                  return;
                }
                if msg.content.to_uppercase().contains("JED") && msg.content.to_uppercase().contains("?") {
                  let _ = msg.reply_ping(&ctx.http, "yeeyee ".to_owned() + msg.author.name.as_str()).await;
                  return;
                }
                if msg.content.to_uppercase().contains("READY") {
                  let _ = msg.reply_ping(&ctx.http, "let's get started ".to_owned() + msg.author.name.as_str()).await;
                  return;
                }
                
                if msg.content.to_uppercase().contains("HELLO THERE"){
                  let _ = msg.reply_ping(&ctx.http,"General Kenobi").await;
                  return;
                }
                if msg.content.to_uppercase() == msg.content && msg.content.to_uppercase() != "AES" {
                  let _ = msg.reply_ping(&ctx.http,"I can hear voices at the back of the room...").await;
                  return;
                }
                if msg.content.to_uppercase() == "AES"{
                  let _ = msg.reply_ping(&ctx.http,"Ummm so .... yeahhhh. I think ahh.. that you forgot a keeyyyy when computing your value.").await;
                  return;
                }                  
                // reads for any substring of 'jed' regardless of case
                if msg.content.to_uppercase().contains(" JED ") ||
                msg.content.to_uppercase().starts_with("JED ") ||
                msg.content.to_uppercase().ends_with(" JED") ||
                msg.content.to_uppercase() == "JED" {
                    // reply with what followed by the name
                    let _ = msg.reply_ping(&ctx.http, "what ".to_owned() + msg.author.name.as_str()).await;
                }
                else if msg.content.to_uppercase().contains("JED") {
                    let _ = msg.reply_ping(&ctx.http, "it's jed").await;
                }
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
