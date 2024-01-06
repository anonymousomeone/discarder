use reqwest;
use serde::Serialize as Ser;

use super::embed::Embed;

pub async fn post(embeds: Vec<Embed>) {
    let client = reqwest::Client::new();

    let webhook = 
        "https://discord.com/api/webhooks/1143404843940200558/zHo8ZnU41WsTVquf1g9mkOn-SJIiqv6UwyfTW0iUnZY6x_TOLNib-QSz3P7Pca7CPdMl"
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
