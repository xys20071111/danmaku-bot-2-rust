use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait DanmakuEventCallback: Send + 'static {
    async fn on_connect(&self);
    async fn on_disconnect(&self);
    // TODO: 完善函数定义，得把弹幕事件的信息传进去
    async fn on_danmaku_event(&self, msg: Vec<Value>) -> Result<()>;
}
