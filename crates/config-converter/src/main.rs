use std::{
    fs::File,
    io::{Read, Write},
    process::exit,
};

use clap::{Command, arg};
use serde_json::{Value, json};

fn main() {
    let cmd = Command::new("Danmaku bot config converter")
        .arg(arg!(-i --input <FILE> "Original config file.").required(true))
        .arg(arg!(-o --output <FILE> "Output file.").required(true))
        .get_matches();

    let mut input_file = File::open(cmd.get_one::<String>("input").unwrap()).unwrap();
    let mut output_file = match File::create_new(cmd.get_one::<String>("output").unwrap()) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("File already exists!");
            exit(1);
        }
    };

    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input);
    let input = serde_json::from_str::<Value>(&input).unwrap();
    let mut rooms: Vec<i64> = Vec::new();
    for item in input["rooms"].as_array().unwrap() {
        if item.is_number() {
            rooms.push(item.as_i64().unwrap());
        } else {
            rooms.push(item["room_id"].as_i64().unwrap());
        }
    }
    let _ = output_file
        .write(
            json!({
                "credential": input["verify"],
                "rooms": rooms
            })
            .to_string()
            .as_bytes(),
        )
        .unwrap();
}
