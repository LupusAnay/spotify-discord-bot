use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group,
    },
};

use std::env;
use songbird::input::{ffmpeg_optioned};
use songbird::{SerenityInit, ffmpeg};

#[group]
#[commands(play)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        // let source = match ffmpeg_optioned("./spotify-test/track", &[], &[
        //     "-f",
        //     "f32",
        //     "-ac",
        //     "1",
        //     "-ar",
        //     "48000",
        //     "-vorbis",
        //     "pcm_f32",
        //     "-",
        let source = match ffmpeg("spotify-test/sine.pcm").await {
            Ok(source) => source,
            Err(why) => {
                println!("Err reading ffmpeg file: {}", why);
                return Ok(());
            }
        };
        handler.play_source(source);

        println!("Playing song");
    } else {
        println!("Not in a voice channel to play in");
    }

    Ok(())
}