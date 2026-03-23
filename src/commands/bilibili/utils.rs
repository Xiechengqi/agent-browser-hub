use md5::{Md5, Digest};

const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35,
    27, 43, 5, 49, 33, 9, 42, 19, 29, 28, 14, 39, 12, 38, 41, 13,
    37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4,
    22, 25, 54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

pub fn get_mixin_key(img_key: &str, sub_key: &str) -> String {
    let combined = format!("{}{}", img_key, sub_key);
    let mut result = String::new();
    for &idx in &MIXIN_KEY_ENC_TAB {
        if idx < combined.len() {
            result.push(combined.chars().nth(idx).unwrap());
        }
    }
    result.chars().take(32).collect()
}

pub fn wbi_sign(params: &mut std::collections::HashMap<String, String>, img_key: &str, sub_key: &str) -> String {
    let mixin_key = get_mixin_key(img_key, sub_key);
    let wts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    params.insert("wts".to_string(), wts);

    let mut keys: Vec<_> = params.keys().cloned().collect();
    keys.sort();

    let query: Vec<String> = keys.iter()
        .map(|k| format!("{}={}", k, urlencoding::encode(&params[k]).replace("+", "%20")))
        .collect();
    let query_str = query.join("&");

    let to_sign = format!("{}{}", query_str, mixin_key);
    let mut hasher = Md5::new();
    hasher.update(to_sign.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
