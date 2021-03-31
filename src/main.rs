extern crate serenity;
extern crate scraper;
extern crate tokio;

use tokio::task; 

use serenity::prelude::*;
use serenity::model::gateway::Ready; 
use serenity::model::channel::Message; 
use serenity::async_trait; 

mod webscrapper; 

struct Handler;

// get new token from discord
const DISCORD_TOKEN : &str = ""; 

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self,_ctx: Context, _new_message: Message) {
        if _new_message.content == "!gallerist" {
            
            let _task = task::spawn(webscrapper::object_of_the_day("https://www.slam.org/explore-the-collection/object-of-the-day/")).await;
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