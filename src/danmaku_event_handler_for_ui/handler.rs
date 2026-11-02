use crate::danmaku_receiver::receiver::DanmakuEventCallback;
use async_trait::async_trait;
use serde_json::Value;

pub struct DanmakuEventHandler {}

#[async_trait]
impl DanmakuEventCallback for DanmakuEventHandler {
    async fn on_connect(&self) {
        // TODO: 设法将事件发送到UI
    }
    async fn on_disconnect(&self) {
        // TODO: 设法将事件发送到UI
    }
    async fn on_danmaku_event(&self, msg: Vec<Value>) {
        println!("{:?}", msg);
        // TODO: 设法将事件发送到UI
    }
}
