use reqwest;
use serde::Serialize as Ser;

use super::embed::Embed;

pub async fn post(embeds: Vec<Embed>) {
    let client = reqwest::Client::new();

    let webhook = 
        "https://discord.com/api/webhooks/1091134777060491324/EHEP3NnVImMQxS_iXKMfFMhjAZ3yG8R-gM532ssTuxOnFe-eToHDHfQC8j5aAAbnXAfE"
        .to_string();

    let data =
    Webhook {
        embeds
    };

    let req = client.post(webhook)
                            .json(&data);

    let _res = req.send().await;

}

#[derive(Ser, Debug)]
struct Webhook {
    // content: String,
    embeds: Vec<Embed>
}