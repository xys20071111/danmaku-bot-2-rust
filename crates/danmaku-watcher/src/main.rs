use std::io;

use anyhow::{Result, bail};
use serde_json::Value;

fn main() -> Result<()> {
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
