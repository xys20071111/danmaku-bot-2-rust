use crate::{
    config::{Credential, ProgramConfig},
    danmaku_handler::DamnakuHandler,
    danmaku_receiver::receiver::DanmakuReceiver,
    plugin::PluginManager,
};
use anyhow::{Result, bail};
use clap::{Command, arg};
// use gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use serde_json::Value;
use std::{fs::File, io::Read, time::Duration};
use tokio::sync::mpsc;

mod config;
mod danmaku_handler;
mod danmaku_receiver;
mod plugin;
mod search_param;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Command::new("Danmaku bot")
        .arg(arg!(-c --config <FILE> "Program config file.").required(true))
        .arg(arg!(-u --ui "Use GUI."))
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
    let mut plugin_manager = PluginManager::new();
    let mut join_handlers: Vec<_> = Vec::new();
    let (tx, rx) = mpsc::channel::<Value>(32);
    let handler = DamnakuHandler::new(tx);
    let _ = plugin_manager.run_plugins(rx).await;
    for room in config.rooms.iter() {
        let cred = cred.clone();
        let room = room.clone();
        let handler = handler.clone();
        let job = tokio::task::spawn(async move {
            loop {
                let room = room.to_owned();
                let _ = DanmakuReceiver::connect(cred.to_owned(), room, handler.clone()).await;
                eprintln!("[{}] Disconnected from server. Reconnect after 30s.", room);
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
        join_handlers.push(job);
    }
    match tokio::signal::ctrl_c().await {
        Err(_) => bail!("Cannot listen signal SIGINT"),
        Ok(_) => println!("Exitting..."),
    }
    Ok(())
}
