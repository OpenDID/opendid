use std::env;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use tokio::runtime;
    let mut rt = runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(future)
}

pub fn get_home_str() -> Option<String> {
    env::var("HOME").ok()
}

pub fn resolve_file_path(path: &str) -> String {
    let home_path = match get_home_str() {
        Some(p) => p, None => return path.to_owned(),
    };
    match path {
        "~" => home_path,
        p if p.starts_with("~/") => home_path + &path.chars().skip(1).collect::<String>(),
        p => p.to_owned(),
    }
}

pub fn decode_block_base64(s: &str) -> Option<Vec<u8>> {
    let mut b64 = String::new();
    for c in s.chars() {
        match c {
            ' ' | '\t' | '\r' | '\n' => (),
            _ => b64.push(c),
        }
    }
    base64::decode(b64).ok()
}

pub fn encode_block_base64(b: &[u8], block_len: usize, block_count: usize) -> String {
    let block_len = if block_len == 0 { 6 } else { block_len };
    let block_count = if block_count == 0 { 10 } else { block_count };
    let s_base64 = base64::encode(b);
    let mut last_block_len = 0;
    let mut last_block_count = 0;
    let s_base64_len = s_base64.len();
    let mut ret = String::with_capacity(s_base64_len + (s_base64_len / block_len) + 16);
    for c in s_base64.chars() {
        if last_block_len == block_len {
            last_block_count += 1;
            if last_block_count == block_count {
                last_block_count = 0;
                ret.push('\n');
            } else {
                ret.push(' ');
            }
            last_block_len = 0;
        }
        ret.push(c);
        last_block_len += 1;
    }
    ret
}

#[test]
fn test_get_home_str() {
    assert_eq!(true, get_home_str().is_some());
}
