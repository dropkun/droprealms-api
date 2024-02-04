
use std::collections::HashMap;
use std::env;
pub async fn send_discord_webhook(msg: String) -> Result<(), reqwest::Error> {
    let webhook_url = env::var("DISCORD_WEBHOOK").expect("Expected a url in the environment");

    let mut map = HashMap::new();
    map.insert("content", msg);
    map.insert("username", String::from("droprealms-api"));
    map.insert("avatar_url", String::from("https://github.com/google.png"));

    let client = reqwest::Client::new();
    let response = client.post(webhook_url).json(&map).send().await?;

    println!("Response: {:?}", response);

    Ok(())
}
