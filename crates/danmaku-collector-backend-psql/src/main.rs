use std::{
    fs::File,
    io::{self, Read},
};

use anyhow::{Result, bail};
use postgres::{Client, NoTls};
use serde_json::{Value, json};

use crate::{config::Config, msg_parse_result::MsgParseResult};

mod config;
mod msg_parse_result;

fn main() -> Result<()> {
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
    let mut psql = match Client::connect(
        &format!(
            "postgres://{}:{}@{}:{}/{}?application_name=danmaku-collector",
            config.username, config.password, config.host, config.port, config.database
        ),
        NoTls,
    ) {
        Ok(val) => val,
        Err(e) => bail!(format!("Cannot connect to database! {}", e)),
    };
    let table = config.table;
    let user = config.username;
    let _ = psql.batch_execute(&format!(
        r"
        CREATE TABLE IF NOT EXISTS {table}
          (
            id bigserial NOT NULL,
            time timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
            room bigint NOT NULL,
            type character varying NOT NULL,
            uid bigint NOT NULL,
            nickname character varying NOT NULL,
            medal jsonb NOT NULL,
            data jsonb NOT NULL,
            PRIMARY KEY (id)
          );
          ALTER TABLE IF EXISTS {table}
              OWNER to {user};
          CREATE INDEX IF NOT EXISTS quick_select
            ON {table} USING btree
            (room ASC NULLS LAST, uid ASC NULLS LAST, nickname ASC NULLS LAST, type ASC NULLS LAST)
            WITH (deduplicate_items=True)
          ;
        "
    ));
    loop {
        let mut data = String::new();
        let _ = io::stdin().read_line(&mut data);
        let msg = serde_json::from_str::<Value>(&data).unwrap();
        let room = msg["room"].as_i64().unwrap();
        let msg: MsgParseResult = match msg["cmd"].as_str().unwrap() {
            "DANMU_MSG" => {
                let mut result = MsgParseResult::default();
                result.event_type = String::from("DANMAKU");
                if !msg["data"][3].is_null() {
                    result.medal_level = match msg["data"][3][0].as_i64() {
                        Some(val) => val,
                        None => 0,
                    };
                    result.medal_name = match msg["data"][3][1].as_str() {
                        Some(val) => String::from(val),
                        None => String::new(),
                    };
                    result.medal_owner = match msg["data"][3][12].as_i64() {
                        Some(val) => val,
                        None => 0,
                    }
                }
                result.uid = msg["data"][2][0].as_i64().unwrap();
                result.nickname = String::from(msg["data"][2][1].as_str().unwrap());
                result.extra_data = json!({
                    "text": msg["data"][1]
                });

                result
            }
            "SEND_GIFT" => {
                let mut result = MsgParseResult::default();
                result.medal_level = msg["data"]["medal_info"]["medal_level"].as_i64().unwrap();
                result.medal_owner = msg["data"]["medal_info"]["target_id"].as_i64().unwrap();
                result.medal_name =
                    String::from(msg["data"]["medal_info"]["medal_name"].as_str().unwrap());
                result.uid = msg["data"]["uid"].as_i64().unwrap();
                result.nickname = String::from(msg["data"]["uname"].as_str().unwrap());
                result.event_type = String::from("GIFT");
                result.extra_data = json!({
                    "count": msg["data"]["super_gift_num"],
                    "name": msg["data"]["giftName"],
                    "totalPrice": msg["data"]["price"].as_f64().unwrap() / 1000f64 * msg["data"]["super_gift_num"].as_f64().unwrap()
                });
                result
            }
            "GUARD_BUY" => {
                let mut result = MsgParseResult::default();
                result.uid = msg["data"]["uid"].as_i64().unwrap();
                result.nickname = String::from(msg["data"]["username"].as_str().unwrap());
                result.event_type = String::from("GUARD");
                result.extra_data = json!({
                "type": msg["data"]["gift_name"],
                "count": msg["data"]["num"]
                });
                result
            }
            "SUPER_CHAT_MESSAGE" => {
                let mut result = MsgParseResult::default();
                result.uid = msg["data"]["uid"].as_i64().unwrap();
                result.nickname = String::from(msg["data"]["user_info"]["uname"].as_str().unwrap());
                result.event_type = String::from("SUPERCHAT");
                if !msg["data"]["medal_info"].is_null() {
                    result.medal_name = match msg["data"]["medal_info"]["medal_name"].as_str() {
                        Some(val) => String::from(val),
                        None => String::new(),
                    };
                    result.medal_level = match msg["data"]["medal_info"]["medal_level"].as_i64() {
                        Some(val) => val,
                        None => 0,
                    };
                    result.medal_owner = match msg["data"]["medal_info"]["target_id"].as_i64() {
                        Some(val) => val,
                        None => 0,
                    };
                }
                result.extra_data = json!({
                    "price": msg["data"]["price"],
                    "text": msg["data"]["message"]
                });
                result
            }
            "INTERACT_WORD" => {
                let mut result = MsgParseResult::default();
                result.event_type = String::from("EVENT");
                result.uid = msg["data"]["uid"].as_i64().unwrap();
                result.nickname = String::from(msg["data"]["uname"].as_str().unwrap());
                result.extra_data = json!({
                  "type": 0
                });
                if !msg["data"]["fans_medal"].is_null() {
                    result.medal_name = match msg["data"]["fans_medal"]["medal_name"].as_str() {
                        Some(val) => String::from(val),
                        None => String::new(),
                    };
                    result.medal_level = match msg["data"]["fans_medal"]["medal_level"].as_i64() {
                        Some(val) => val,
                        None => 0,
                    };
                    result.medal_owner = match msg["data"]["fans_medal"]["target_id"].as_i64() {
                        Some(val) => val,
                        None => 0,
                    };
                }
                result
            }
            "ROOM_BLOCK_MSG" => {
                let mut result = MsgParseResult::default();
                result.event_type = String::from("EVENT");
                result.uid = msg["data"]["uid"].as_i64().unwrap();
                result.nickname = String::from(msg["data"]["uname"].as_str().unwrap());
                result.extra_data = json!({
                  "type": 1
                });
                result
            }
            _ => continue,
        };
        match psql.execute(
            &format!(
                "INSERT INTO {table}(room,type,uid,nickname,medal,data) VALUES ($1,$2,$3,$4,$5,$6)"
            ),
            &[
                &room,
                &msg.event_type,
                &msg.uid,
                &msg.nickname,
                &json!({"medal_name": msg.medal_name, "medal_owner": msg.medal_owner, "medal_level": msg.medal_level}),
                &msg.extra_data,
            ],
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
