use serde_json::Value;

#[derive(Default)]
pub struct MsgParseResult {
    pub medal_name: String,
    pub medal_level: i64,
    pub medal_owner: i64,
    pub nickname: String,
    pub uid: i64,
    pub event_type: String,
    pub extra_data: Value,
}
