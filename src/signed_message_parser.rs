use crate::util::XResult;

const FIVE_MINUS: &str = "-----";

pub struct Header {
    key: String,
    value: String,
}

pub struct DidSignedMessage {
    message: Vec<u8>,
    raw_message: String,
    signed_headers: Vec<Header>,
    signed_signature: Vec<u8>,
}

impl DidSignedMessage {

    pub fn parse(s: &str) -> XResult<Self> {
        let mut message = vec![];
        let mut raw_message = String::new();
        let mut signed_headers = vec![];
        let mut signed_signature = vec![];

        let mut is_in_message = false;
        let mut is_in_signed = false;
        for ln in s.lines() {
            let ln = ln.trim();
            // TODO ...
            if is_in_message {

            } else if is_in_signed {

            } else {
                if let Some(s) = extra_cmd_line(ln) {
                    // TODO ...
                }
            }
        }

        Ok(Self {
            message,
            raw_message,
            signed_headers,
            signed_signature,
        })
    }
}

fn extra_cmd_line(ln: &str) -> Option<String> {
    if ln.starts_with(FIVE_MINUS) && ln.ends_with(FIVE_MINUS) {
        let mut all_c = vec![];
        for c in ln.chars() {
            all_c.push(c);
        }
        let mut head_minus_count = 0;
        for i in 0..all_c.len() {
            if all_c[i] == '-' {
                head_minus_count += 1;
            } else {
                break;
            }
        }
        let mut foot_minus_count = 0;
        let mut i = all_c.len() - 1;
        loop {
            if i == 0 { break; }
            if all_c[i] == '-' {
                foot_minus_count += 1;
            } else {
                break;
            }
            i -= 1;
        }
        let mut ret = String::new();
        let mut lastc = None;
        for i in head_minus_count..all_c.len() - foot_minus_count {
            let c = all_c[i];
            let ignore_c = if let Some(last_c) = lastc {
                if last_c == ' ' && c == ' ' { true } else { false }
            } else {
                false
            };
            if !ignore_c {
                ret.push(c);
            }
            lastc = Some(c);
        }
        Some(ret.trim().to_owned())
    } else {
        None
    }
}

#[test]
fn test_extra_cmd_line() {
    assert_eq!(None, extra_cmd_line(""));
    assert_eq!(None, extra_cmd_line("aaaa"));
    assert_eq!(None, extra_cmd_line("-a-"));
    assert_eq!(None, extra_cmd_line("--a--"));
    assert_eq!(None, extra_cmd_line("---a---"));
    assert_eq!(None, extra_cmd_line("----a----"));
    assert_eq!(Some("a".to_owned()), extra_cmd_line("-----a-----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a b-----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a b -----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a   b -----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("----- a   b -----"));
}