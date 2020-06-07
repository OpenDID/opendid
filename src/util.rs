use std::env;

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

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
