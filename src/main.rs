extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready; 
use serenity::model::channel::Message; 
use serenity::async_trait; 

use std::env; 

struct Handler;

const DISCORD_TOKEN : &str = "ODI2MDM3NzQ0NzE1MzAwODc1.YGGpmQ.eGDl-VVgKFGX8rl958HAYv8rEG4"; 

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self,_ctx: Context, _new_message: Message) {
        if _new_message.content == "!gallerist" {
            if let Err(why) = _new_message.channel_id.say(&_ctx.http, "test success!").await {
                println!("Error giving message: {:?}", why);
            }
        }
    }

    async fn ready(&self,_ctx: Context, ready: Ready){
        println!("{} is ready", ready.user.name);
    }
}

#[tokio::main]

async fn main () {
    
    
    let mut client = Client::builder(&DISCORD_TOKEN)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}