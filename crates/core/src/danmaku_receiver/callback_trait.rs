use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait DanmakuEventCallback: Send + 'static {
    async fn on_connect(&self);
    async fn on_disconnect(&self);
    async fn on_danmaku_event(&self, msg: Vec<Value>) -> Result<()>;
}
