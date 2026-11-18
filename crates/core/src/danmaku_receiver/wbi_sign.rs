use crate::search_param::SearchParam;
use md5::{Digest, Md5};
use serde_json::Value;

const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29,
    28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25,
    54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

async fn get_wbi_keys() -> String {
    let req = reqwest::get("https://api.bilibili.com/x/web-interface/nav")
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    let img_key = req["data"]["wbi_img"]["img_url"]
        .as_str()
        .unwrap()
        .strip_suffix(".png")
        .unwrap()
        .strip_prefix("https://i0.hdslb.com/bfs/wbi/")
        .unwrap();
    let sub_key = req["data"]["wbi_img"]["sub_url"]
        .as_str()
        .unwrap()
        .strip_suffix(".png")
        .unwrap()
        .strip_prefix("https://i0.hdslb.com/bfs/wbi/")
        .unwrap();
    let img_key = String::from(img_key);
    let sub_key = String::from(sub_key);
    String::from(format!("{}{}", img_key, sub_key))
}

async fn gen_mixin_key() -> String {
    let raw_wbi_key = get_wbi_keys().await;
    let raw_wbi_key = raw_wbi_key.as_bytes();
    let binding = MIXIN_KEY_ENC_TAB
        .iter()
        // 此步操作即遍历 MIXIN_KEY_ENC_TAB，取出 raw_wbi_key 中对应位置的字符
        .map(|n| raw_wbi_key[n.to_owned()])
        // 并收集进数组内
        .collect::<Vec<u8>>();
    let mut mixin_key = match String::from_utf8(binding) {
        Ok(val) => val,
        Err(_) => panic!("Cannot read wbi key!"),
    };
    let _ = mixin_key.split_off(32); // 截取前 32 位字符
    mixin_key
}

pub async fn wbi_sign(param: Vec<(String, String)>) -> Vec<(String, String)> {
    let key = gen_mixin_key().await;
    let mut search_params = SearchParam::from_pairs(param);
    search_params.sort();
    let mut hasher = Md5::new();
    hasher.update((search_params.to_string() + key.as_str()).as_bytes());
    let hash = hasher.finalize().to_vec();
    let hash: Vec<String> = hash.iter().map(|val| format!("{:02X}", val)).collect();
    search_params.append(String::from("w_rid"), hash.join("").to_ascii_lowercase());
    search_params.to_vec().to_owned()
}
