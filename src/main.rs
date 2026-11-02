use std::{fs::File, io::Read};

use crate::{
    config::{Credential, ProgramConfig},
    danmaku_event_handler_for_ui::handler::DanmakuEventHandler,
    danmaku_receiver::receiver::DanmakuReceiver,
};
use anyhow;
use clap::{Command, arg};

mod config;
mod danmaku_event_handler_for_ui;
mod danmaku_receiver;
mod search_param;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cmd = Command::new("Danmaku bot")
        .arg(arg!(-c --config <FILE> "Program config file.").required(true))
        .get_matches();
    let config: String = cmd.get_one::<String>("config").unwrap().to_owned();
    let mut config = File::open(config).expect("Cannot open config file.");
    let mut config_content = String::new();
    let _ = config.read_to_string(&mut config_content);
    let config: ProgramConfig = serde_json::from_str::<ProgramConfig>(&config_content)
        .expect("Cannot parse config file. Check with your editor.");
    let cred = &config.credential;
    let cred = Credential {
        access_key: cred.access_key.to_owned(),
        cookies: cred.cookies.to_owned(),
        uid: cred.uid,
    };
    let handler = DanmakuEventHandler {};
    let _ = DanmakuReceiver::connect(cred, 25755118, handler).await;

    Ok(())
}
