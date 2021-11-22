use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, channel::Reaction, channel::ReactionType},
    prelude::*,
};
use std::fs::OpenOptions;
use std::io::Write;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // dont read bot messages
        if !msg.author.bot {
            // message log
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("log.txt")
                .unwrap();

            if let Err(e) = writeln!(file, "{} : {} ", msg.author.name.as_str(), msg.content) {
                eprintln!("Couldn't write to file: {}", e);
            }

            // only responds in #bot-commands
            if msg.channel_id.eq(&879561100260151356) {                
                if msg.content.to_uppercase().contains("JED")
                    && msg.content.to_uppercase().contains("?")
                {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "yeeyee :point_right: ".to_owned() + msg.author.name.as_str(),
                        )
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("SUSSY") {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            SUSSY.to_owned() // i c that now
                            
                        )
                        .await;
                    return;
                }
                // start of message chains - Chicken
                if msg.content.to_uppercase().contains("READY") {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "let's get started ".to_owned() + msg.author.name.as_str(),
                        )
                        .await;
                    return;
                }

                if msg.content.to_uppercase().contains("HELLO THERE") {
                    let _ = msg.reply_ping(&ctx.http, "General Kenobi").await;
                    return;
                }

                
                if msg.content.to_uppercase().contains("MAGIC") {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "Want to see a magic trick? ".to_owned()
                                + msg.author.name.as_str()
                                + "\n '>yes' \n '>no' ",
                        )
                        .await;
                }
                if msg.content.to_uppercase() == ">YES" {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "I lost your MD5 hash and your assignment grades with it",
                        )
                        .await;
                }
                if msg.content.to_uppercase().contains("FIX") {
                    let _ = msg.reply_ping(&ctx.http, "I'll fix it later").await;
                }
                if msg.content.to_uppercase() == ">NO" {
                    let _ = msg
                        .reply_ping(&ctx.http, "I guess you'll find out later")
                        .await;
                }

                if msg.content.to_uppercase().contains("JOKE") {
                    let _ = msg.reply_ping(&ctx.http,"SQL walks into a bar, then it walks out... \n Because there was no tables!😂").await;
                }
                // Saajan - MythicalEngineer
                if msg.content.to_uppercase().contains("I LIKE MAC")
                    || msg.content.to_uppercase().contains("I LIKE WINDOWS")
                {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "🤬Only linux on my watch ".to_owned() + msg.author.name.as_str(),
                        )
                        .await;
                    return;
                }
                if msg.content.to_uppercase() == "I WALKED INTO THE ROOM" {
                    let _ = msg
                        .reply_ping(
                            &ctx.http,
                            "And you saw me, pacing furiously around, stretching my arms above my head as I lectured.",
                        )
                        .await;
                }
                // Saajan - MythicalEngineer
                if msg.content.to_uppercase() == "AES" {
                    let _ = msg.reply_ping(&ctx.http,"Ummm so .... yeahhhh. I think ahh.. that you forgot a keeyyyy when computing your value.").await;
                    return;
                }
                // Saajan - MythicalEngineer
                if msg.content.to_uppercase().contains("MAKE A PIPE") {
                    let _ = msg
                        .reply_ping(&ctx.http, "Here you go: my_unnamed_pipe👍")
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("ADVICE") {
                    let _ = msg
                        .reply_ping(&ctx.http, "It is better to be a warrior in a garden than a garden in a war.")
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("MAKE A PIPE") {
                    let _ = msg
                        .reply_ping(&ctx.http, "Here you go: my_unnamed_pipe👍")
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("HEAVY DETAIL") {
                    let _ = msg
                        .reply_ping(&ctx.http, "I will, I WILL GO INTO HEAVY DETAIL.")
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("CALM DOWN") {
                    let _ = msg
                        .reply_ping(&ctx.http, "NO.")
                        .await;
                    return;
                }
                if msg.content.to_uppercase().contains("GIVE") {
                    let _ = msg
                        .reply_ping(&ctx.http, "no")
                        .await;
                    return;
                }
                // reads for any substring of 'jed' regardless of case
                if msg.content.to_uppercase().contains(" JED ")
                    || msg.content.to_uppercase().starts_with("JED ")
                    || msg.content.to_uppercase().ends_with(" JED")
                    || msg.content.to_uppercase() == "JED"
                    || msg.content.to_uppercase().contains(" JEDI ")
                    || msg.content.to_uppercase().starts_with("JEDI ")
                    || msg.content.to_uppercase().ends_with(" JEDI")
                    || msg.content.to_uppercase() == "JEDI"
                {
                    // reply with what followed by the name
                    let _ = msg
                        .reply_ping(&ctx.http, "what ".to_owned() + msg.author.name.as_str())
                        .await;
                } else if msg.content.to_uppercase().contains("JED") {
                    let _ = msg.reply_ping(&ctx.http, "it's jed").await;
                }

                if msg.content.to_uppercase() == "HOW WAS YOUR DAY" {
                    let _ = msg.reply_ping(&ctx.http,"these kids are tough. I got 600 students in three sections.").await;
                    return;
                }

                let s = msg.content.chars().filter(|c| c.is_alphabetic()).collect::<String>();

                if s.chars().all(char::is_uppercase)
                && !(s.is_empty())
                {
                    let _ = msg
                        .reply_ping(&ctx.http, "I can hear voices at the back of the room...")
                        .await;
                    return;
                }

                // NO COMMANDS AFTER THIS COMMENT (keep commands in bot channel)
            }
        }

    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
    if add_reaction.channel_id.eq(&879561100260151356) {
          match add_reaction.emoji{
              ReactionType::Unicode(ref s) => {
                  let c = s.chars().nth(0).unwrap();
                  if c == '😂' || c == '🤣' || c == '😆' {
                      match add_reaction.user_id{
                        Some(ref id) =>{
                            let mut message = "Ummm... soo yeahh, I think we should keep it a bit more respectful in here <@".to_owned();
                            message.push_str(&id.as_u64().to_string());
                            message.push_str(">");
                            let _ = add_reaction.channel_id.say(&ctx.http,message).await;
                        },
                        _ => {}
                      }
                  }
              },
              _ => {}
          };
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
const SUSSY: &str ="```fix
⠀ ⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⣀⣤⣤⣤⣤⣤⣤⣤⣤⣄⣀   
 ⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡀     
⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦    
⠀ ⠀⠀⠀⠀⠀⠀⠀⠀ ⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷    
⠀ ⠀⠀⠀⠀⠀ ⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⠛⠛⠛⠛⠿⠿⣿⣿⣷⣄   
⠀ ⠀⠀⠀ ⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀ ⠀⠀⠀⠀ ⠀ ⠀⠈⠻⣿⣷     
⠀⠀ ⢀⣠⣤⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀ ⠀⠀⠀⠀⠀⠀⠀ ⠀ ⢸⣿⡇     
⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣤⣤⣤⣤⣤⣤⣤⣴⣶⣿⣿⡿     
⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃   
⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▄▄▀▀█⣿⣿⣿⣿⣿⠁    
⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▄█▒░░▄░█⣿⣿⣿⣿    
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▄▀▒▀▀▀▄▄▀⣿⣿⣿⣿    
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿█▒░░░░░█⣿⣿⣿⣿⣿⣿    
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ █▒░░░░░█⣿⣿⣿⣿⣿⣿⡇    
⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿█▒▒░░░▒█⣿⣿⣿⣿⣿⣿⡿    
⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▓▓▒▒▒▀▀▀█▄⣿⣿⣿⣿⣿⡇   
⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▓▒▒▒▒▒▒▒▒▒█⣿⣿⣿⣿⡇   
⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿▓▒▒▒▒▒▒▓▒▒█⣿⣿⣿⣿   
⠀⠀⠀⠙⠿⠿⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿▀▄▄▄▄█▄▄▀⣿⣿⣿⣿⣿   
⠀⠀⠀⠀⠀⠀ ⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇   
⠀⠀⠀⠀⠀⠀⠀ ⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⢐⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃   
⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⡿⣟⣯⣿⠟⡉⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⢿⣽⣿⣿⣿⠿⠿⠟⠒⠉⠉⠉⠉⠉⠉⠉⠙⠋   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠿⠋⠉⢀⣠⣤⣤⡔⣄   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⠾⠛⠋⠉⠀⢀⣀⠐⣤⣶⣶⡤⢤⣤   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣰⣶⣾⣿⣿⣿⣆⠀⣀⣀⡀⣀⡀   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠀⢀⢀⣀⠀⣀⣈⡿⠿⠿⠽⠃   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠿⠿⠿⠿⠾⠟⢁⣀⡴⣦⠆   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢦⣤⣀⣀⠀⠀⠀⠀⢘⣿⣍⡷⠆   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢶⣄⠈⠉⠛⠛⠿⠓⠀⠉⠋⠉⣀   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⡀⠙⠻⢶⣶⡤⠀⠀⠛⠶⠾⠼⠋   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣆⠈⠻⣶⣤⡀⠀⠀⢸⠿⣶⣦⣤⣠⣾   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠙⢷⣤⣀⠈⠁⠀⠀⢠⣤⣀⠈⠉⠈   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡌⢧⣀⠉⠛⠃⠀⠀⠀⠀⠉⠛⠿⠿⠻⠃   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⢳⣄⠙⠛⢋⠁⠀⠀⠀⠘⠿⣴⣤⣄⣤⡄   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣄⡙⠛⠋⠀⠀⠀⠀⠀⠰⣤⣀⠉⠉⠉⠀   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢠⡈⠉⠉⠀⠀⠀⠀⠀⠀⢀⡈⠙⠛⠁   
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢦⡉⠛⡁⠀⠀⠀⠀⠀⠀⠈⠻⠷⣶⣦⡆
⠀⠀⠀⠀⠀⠀⠀⠀⢠⡈⢷⣌⠙⠛⠁⠀⠀⠀⠀⠀⠀⠰⣦⣄⣀⣀⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⣄⡉⠛⠛⠀⠀⠀⠀⠀⠀⠀⢀⠈⠙⠛⠛⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢦⣀⠉⠛⠷⠖⠀⠀⠀⠀⠀⠀⠀⠘⠿⣶⣦⡄
⠀⠀⠀⠀⠀⠀⠀⣠⣀⠙⠳⠶⠶⠀⠀⠀⠀⠀⠀⠀⠀⢠⣀⣀⣀
⠀⠀⠀⠀⠀⠀⠀⠙⠻⢿⣶⣤⣤⠀⠀⠀⠀⠀⠀⠀⢠⠛⠛⠻⠿
⠀⠀⠀⠀⠀⠀⠀⠰⣦⣄⠈⠉⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈
⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣶⡆⠀⠀⠀⠀⠀⠀⠀⠺⠿⠿⠿⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⠟⠁⠀⠀⠀⠀⠀⠀⢀⣤⣤⣤⣤⡄
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣀⣀⣀⣀⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⠻⠿⠿⠧
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣞⣻⣿⣿⣔⣿⠂  
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠋⠉⠉⠁```";
