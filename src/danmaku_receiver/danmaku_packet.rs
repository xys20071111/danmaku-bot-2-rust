use std::io::Read;

use anyhow::{Result, bail};
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::Message;

pub async fn packet_generator(protocol: u16, packet_type: u32, payload: &[u8]) -> Message {
    let packet_length = 16 + payload.len();
    let mut packet_buffer: Vec<u8> = Vec::with_capacity(packet_length);
    let _ = packet_buffer.write_i32(packet_length as i32).await;
    let _ = packet_buffer.write_i16(16).await;
    let _ = packet_buffer.write_u16(protocol).await;
    let _ = packet_buffer.write_u32(packet_type).await;
    let _ = packet_buffer.write_u32(1).await;
    let _ = packet_buffer.write(payload).await;
    Message::binary(packet_buffer)
}

pub fn packet_extractor(msg: Message) -> Result<(i16, i32, Vec<u8>)> {
    let raw_msg = msg.into_data();
    let raw_msg: Vec<u8> = raw_msg.to_vec();
    let raw_msg: &[u8] = &raw_msg;
    let protocol = i16::from_be_bytes([raw_msg[6], raw_msg[7]]); // raw_msg
    let packet_type = i32::from_be_bytes([raw_msg[8], raw_msg[9], raw_msg[10], raw_msg[11]]);
    let payload: Vec<u8> = Vec::from(&raw_msg[16..]);
    Ok((protocol, packet_type, payload))
}

pub async fn packet_payload_extactor(payload: Vec<u8>) -> Result<Vec<Value>> {
    let mut decompressor = brotli::Decompressor::new(payload.as_slice(), 4096);
    let mut result_buffer: Vec<u8> = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        match decompressor.read(&mut buf[..]) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                result_buffer.append(&mut Vec::from(&buf[..size]));
            }
            Err(e) => {
                if let std::io::ErrorKind::Interrupted = e.kind() {
                    continue;
                }
                bail!("{}", e);
            }
        }
    }
    let mut result_value_vec: Vec<Value> = Vec::new();
    let mut offset: usize = 0;
    while offset < result_buffer.len() {
        let length = u32::from_be_bytes([
            result_buffer[offset],
            result_buffer[offset + 1],
            result_buffer[offset + 2],
            result_buffer[offset + 3],
        ]);
        let result_json = &result_buffer[offset + 16..offset + usize::try_from(length).unwrap()];
        let result_json = match String::from_utf8(result_json.to_vec()) {
            Ok(val) => val,
            Err(e) => bail!("{}", e),
        };
        let result_json = match serde_json::from_str::<Value>(&result_json) {
            Ok(val) => val,
            Err(e) => bail!("{}", e),
        };
        result_value_vec.push(result_json);
        offset += length as usize;
    }
    Ok(result_value_vec)
}
