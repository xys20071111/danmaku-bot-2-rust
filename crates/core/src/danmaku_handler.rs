use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::danmaku_receiver::callback_trait::DanmakuEventCallback;

#[derive(Clone)]
pub struct DamnakuHandler {
    sender: Sender<Value>,
}

impl DamnakuHandler {
    pub fn new(sender: Sender<Value>) -> Self {
        Self { sender: sender }
    }
}

#[async_trait]
impl DanmakuEventCallback for DamnakuHandler {
    async fn on_connect(&self) {}
    async fn on_disconnect(&self) {}
    async fn on_danmaku_event(&self, msg: Vec<Value>) -> Result<()> {
        for item in msg.iter() {
            let _ = self.sender.send(item.to_owned()).await;
        }
        Ok(())
    }
}
