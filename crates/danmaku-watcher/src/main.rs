use std::{
    fs::File,
    io::{self, Write},
    time::{self, UNIX_EPOCH},
};

use anyhow::{Result, bail};
use serde_json::Value;

fn main() -> Result<()> {
    // 将收到的事件导出到文件中，便于调试
    let mut danmaku_event_log_file = File::create_new(format!(
        "danmaku_event_{}.log",
        time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ))
    .unwrap();
    loop {
        let mut data = String::new();
        let len = match io::stdin().read_line(&mut data) {
            Ok(val) => val,
            Err(_) => bail!("Cannot read json msg!"),
        };
        if len < 1 {
            continue;
        }
        let msg = serde_json::from_str::<Value>(&data).unwrap();
        let _ = danmaku_event_log_file.write(data.as_bytes());
        if msg["cmd"] == "DANMU_MSG" {
            let result = format!(
                "房间 {} {} [{}级{}] {}",
                msg["room"],
                msg["data"][2][1],
                msg["data"][3][0],
                msg["data"][3][1],
                msg["data"][1]
            );
            println!("{}", result);
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
