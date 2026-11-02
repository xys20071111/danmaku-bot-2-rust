use crate::{
    config::Credential,
    danmaku_receiver::{
        danmaku_packet::{packet_extractor, packet_generator, packet_payload_extactor},
        get_cookie_value::get_cookie_value,
    },
};
use anyhow::{self, bail};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use reqwest::Url;
use serde_json::{Value, json};
use std::{
    fmt::Debug,
    time::{self, Duration, UNIX_EPOCH},
};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;

use crate::danmaku_receiver::app_sign::app_sign;

static ROOM_INFO_BASE_URL: &str =
    "https://api.live.bilibili.com/xlive/app-room/v1/index/getDanmuInfo";

impl Debug for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "key: {}\ncookie: {}\nuid: {}\n",
            self.access_key, self.cookies, self.uid
        )
    }
}

#[async_trait]
pub trait DanmakuEventCallback: Send + 'static {
    async fn on_connect(&self);
    async fn on_disconnect(&self);
    // TODO: 完善函数定义，得把弹幕事件的信息传进去
    async fn on_danmaku_event(&self, msg: Vec<Value>);
}

#[derive(Debug)]
pub struct DanmakuReceiver;

impl DanmakuReceiver {
    pub async fn connect<CB: DanmakuEventCallback>(
        cred: Credential,
        room_id: i64,
        callback: CB,
    ) -> Result<(), anyhow::Error> {
        let room_info_pramas = vec![
            (String::from("access_key"), String::from(&cred.access_key)),
            (String::from("room_id"), String::from(&format!("{room_id}"))),
            (String::from("type"), String::from(&format!("{}", 0))),
            (
                String::from("ts"),
                String::from(&format!(
                    "{}",
                    time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                )),
            ),
        ];
        let room_info_params = app_sign(
            room_info_pramas,
            "1d8b6e7d45233436",
            "560c52ccd288fed045859ed18bffd973",
        );
        let client = reqwest::Client::new();
        let url = Url::parse_with_params(ROOM_INFO_BASE_URL, room_info_params)
            .expect("Cannot parse room info url. This shouldn't happened...");
        // println!("{}", url);
        let req = client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        // println!("{}", req);
        if req["code"].as_i64().unwrap() != 0i64 {
            bail!("Failed to get danmaku host info.")
        }
        let (ws, _) = connect_async(format!(
            "wss://{}/sub",
            req["data"]["host_list"][0]["host"].as_str().unwrap()
        ))
        .await
        .unwrap();
        let (mut send, mut recv) = ws.split();
        let (tx, mut rx) = mpsc::channel::<tokio_tungstenite::tungstenite::Message>(32);

        // 2. 启动专门负责网络写入的任务
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = send.send(msg).await {
                    eprintln!("Failed to send ws message: {:?}", e);
                    break;
                }
            }
        });
        let payload = json!({
            "roomid": room_id,
                      "protover": 3,
                      "platform": "web",
                      "uid": &cred.uid,
                      "buvid": get_cookie_value(&cred.cookies, "buvid3").expect("Cannot get buvid3 from cookies."),
                      "key": req["data"]["token"],
                      "type": 2,
        });
        let msg = packet_generator(1, 7, payload.to_string().as_bytes()).await;
        let _ = tx.send(msg).await;
        while let Some(msg) = recv.next().await {
            let raw_msg = match msg {
                Ok(msg) => msg,
                Err(_) => {
                    callback.on_disconnect().await;
                    bail!("[{}] Disconnected by remote server.", room_id)
                }
            };
            if let Ok((protocol, packet_type, payload)) = packet_extractor(raw_msg) {
                match packet_type {
                    // 心跳包回复，不做处理
                    3 => {}
                    // 认证包回复，认证通过，启动心跳
                    8 => {
                        println!("{} passed the auth", room_id);
                        let tx_clone = tx.clone();
                        tokio::spawn(async move {
                            let mut timer = tokio::time::interval(Duration::from_secs(30));
                            loop {
                                let _ = tx_clone
                                    .send(packet_generator(1, 2, &vec![1u8]).await)
                                    .await;
                                timer.tick().await;
                            }
                        });
                        callback.on_connect().await;
                    }
                    // 带弹幕数据的包
                    5 => {
                        //TODO:数据处理部分
                        match protocol {
                            // 原始JSON格式数据
                            0 => {
                                // 对我来说大多没用，先留下处理逻辑吧
                                let payload = &String::from_utf8(payload).unwrap();
                                let _ = serde_json::from_str::<Value>(payload).unwrap();
                                // callback.on_danmaku_event(result).await;
                            }
                            // 经Brotli压缩的批量弹幕数据
                            3 => {
                                let result = match packet_payload_extactor(payload).await {
                                    Ok(val) => val,
                                    Err(e) => bail!("{}", e),
                                };
                                callback.on_danmaku_event(result).await
                            }
                            _ => eprintln!("[{}] Unsupported packet protocol.", room_id),
                        }
                        // callback.on_danmaku_event(packet_data).await;
                    }
                    _ => println!("[{}] Unsupported packet type {}.", room_id, packet_type),
                }
            } else {
                bail!("Failed to get packet info!");
            }
        }
        Ok(())
        // Ok(Self { room_id, cred, ws })
    }
}
