use anyhow::{Result, bail};
use config::Config;
use reqwest::Client;
use serde_json::Value;
use std::{
    fs::File,
    io::{self, Read},
};
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = match File::open("./config.json") {
        Ok(val) => val,
        Err(_) => bail!("Cannot open config file!"),
    };
    let mut config_str = String::new();
    match config.read_to_string(&mut config_str) {
        Err(_) => bail!("Cannot read config file!"),
        Ok(_) => {}
    }
    let config = match serde_json::from_str::<Config>(&config_str) {
        Ok(val) => val,
        Err(_) => bail!("Cannot parse config file! Please check it with your editor."),
    };
    let client = Client::new();
    loop {
        let mut data = String::new();
        let _ = io::stdin().read_line(&mut data);
        let msg = serde_json::from_str::<Value>(&data).unwrap();
        let room_id = msg["room"].as_i64().unwrap();
        if msg["cmd"] == "LIVE" || msg["cmd"] == "PREPARING" {
            let req = match reqwest::get(format!(
                "https://api.live.bilibili.com/xlive/web-room/v1/index/getRoomBaseInfo?room_ids={}&req_biz=BiLive",
                room_id
            )).await {
                Ok(val) => val,
                Err(e) => bail!("Cannot query room info for {}. {:?}", room_id, e),
            };

            let room_info: Value = req.json().await.unwrap();
            if room_info["code"].as_i64().unwrap() != 0 {
                bail!(
                    "Cannot query room info for {}, msg: {}",
                    room_id,
                    room_info.to_string()
                )
            }
            for room in room_info["data"]["by_room_ids"].as_array().unwrap() {
                let status = match msg["cmd"].as_str().unwrap() {
                    "LIVE" => format!(
                        "{} 开播啦！ {}",
                        room["uname"].as_str().unwrap(),
                        room["title"].as_str().unwrap()
                    ),
                    "PREPARING" => format!("{} 下播啦！", room["uname"].as_str().unwrap()),
                    _ => continue,
                };
                let req = client
                    .post(format!("https://{}/{}", config.server, config.topic))
                    .body(format!("{}", status))
                    .header("Authorization", format!("Bearer {}", config.token))
                    .build()
                    .unwrap();
                let _ = client.execute(req).await;
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
