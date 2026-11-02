use md5::{Digest, Md5};

use crate::search_param::SearchParam;

pub fn app_sign(param: Vec<(String, String)>, key: &str, sec: &str) -> Vec<(String, String)> {
    let mut search_params = SearchParam::from_pairs(param);
    search_params.append(String::from("appkey"), key.to_owned());
    search_params.sort();
    let mut hasher = Md5::new();
    hasher.update((search_params.to_string() + sec).as_bytes());
    let hash = hasher.finalize().to_vec();
    let hash: Vec<String> = hash.iter().map(|val| format!("{:02X}", val)).collect();
    search_params.append(String::from("sign"), hash.join("").to_ascii_lowercase());
    return search_params.to_vec().to_owned();
}
