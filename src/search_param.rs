use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct SearchParam {
    pub params: Vec<(String, String)>,
}

impl SearchParam {
    // Create empty
    // pub fn new() -> Self {
    //     Self { params: Vec::new() }
    // }
    // Create new from Header
    pub fn from_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            params: pairs
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
    pub fn append(&mut self, key: String, value: String) {
        self.params.push((key, value));
    }
    pub fn sort(&mut self) {
        self.params.sort_by(|a, b| a.0.cmp(&b.0));
    }
    /// 转换为查询字符串
    pub fn to_string(&self) -> String {
        self.params
            .iter()
            .map(|(key, value)| format!("{}={}", encode_component(key), encode_component(value)))
            .collect::<Vec<_>>()
            .join("&")
    }

    pub fn to_vec(&self) -> &Vec<(String, String)> {
        &self.params
    }
}

/// 实现 Display trait，使其可直接使用 `.to_string()` 或用于 `println!("{}", params)`
impl fmt::Display for SearchParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn encode_component(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}
