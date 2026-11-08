use std::{fs::canonicalize, process::Stdio};

use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::Value;
use tokio::{
    fs,
    io::AsyncWriteExt,
    process::Command,
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

#[derive(Deserialize)]
struct PluginMetadata {
    author: String,
    name: String,
    license: String,
    main_entry: String,
}

pub struct PluginManager {
    jobs: Vec<JoinHandle<()>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub async fn run_plugins(&mut self, mut recv: mpsc::Receiver<Value>) -> Result<()> {
        let (tx, _) = broadcast::channel::<String>(32);
        let mut plugin_dirs = match fs::read_dir("./plugins").await {
            Ok(val) => val,
            Err(_) => {
                let _ = fs::create_dir_all("./plugins").await;
                fs::read_dir("./plugins").await.unwrap()
            }
        };
        loop {
            let plugin = match plugin_dirs.next_entry().await {
                Ok(val) => val,
                Err(_) => bail!("Cannot read plugin dir"),
            };
            if let None = plugin {
                break;
            }
            let plugin = plugin.unwrap().path();
            let metadata_for_plugin =
                fs::read_to_string(&plugin.clone().join("metadata.json")).await;
            let metadata_for_plugin = match metadata_for_plugin {
                Ok(val) => val,
                Err(_) => bail!(
                    "Cannot read metadata for plugin dir \"{}\"",
                    &plugin.file_name().unwrap().to_str().unwrap()
                ),
            };
            let metadata_for_plugin =
                serde_json::from_str::<PluginMetadata>(&metadata_for_plugin).unwrap();
            let plugin_name = metadata_for_plugin.name.clone();
            let executable = metadata_for_plugin.main_entry.clone();
            let mut recv = tx.subscribe();
            let job = tokio::spawn(async move {
                loop {
                    let full_command = &plugin.clone().join(&executable);
                    let working_dir = canonicalize(&plugin).unwrap();
                    let mut subprocess = Command::new(canonicalize(full_command).unwrap());
                    let mut subprocess = subprocess
                        .current_dir(working_dir)
                        .stdin(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let mut stdin = match subprocess.stdin {
                        Some(val) => val,
                        None => {
                            eprintln!("[Plugins] {} closed stdin, stopping...", plugin_name);
                            let _ = subprocess.kill().await;
                            break;
                        }
                    };
                    while let Ok(msg) = recv.recv().await {
                        let _ = stdin.write(msg.to_string().as_bytes()).await;
                    }
                }
            });
            self.jobs.push(job);
        }
        tokio::spawn(async move {
            while let Some(msg) = recv.recv().await {
                let _ = tx.send(format!("{}\n", msg.to_owned()));
            }
        });
        Ok(())
    }
}
