use crate::danmaku_receiver::callback_trait::DanmakuEventCallback;
use anyhow::{Result, bail};
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct DanmakuEventForUI {
    pub send: mpsc::Sender<Vec<Value>>,
}

#[async_trait]
impl DanmakuEventCallback for DanmakuEventForUI {
    async fn on_connect(&self) {
        // TODO: 设法将事件发送到UI
    }
    async fn on_disconnect(&self) {
        // TODO: 设法将事件发送到UI
    }
    async fn on_danmaku_event(&self, msg: Vec<Value>) -> Result<()> {
        match self.send.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => bail!("{}", e),
        }
        // TODO: 设法将事件发送到UI
    }
}
