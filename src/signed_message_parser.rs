// use crate::util::XResult;
use crate::util::decode_block_base64;

const FIVE_MINUS: &str = "-----";
const BEGIN_DID_MESSAGE: &str = "BEGIN DID SIGNED MESSAGE";
const BEGIN_DID_SIGNATURE: &str = "BEGIN DID SIGNATURE";
const END_DID_SIGNATURE: &str = "END DID SIGNATURE";

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct DidSignedMessage {
    pub message: Option<Vec<u8>>,
    pub raw_messages: Vec<String>,
    pub signed_headers: Vec<Header>,
    pub signed_signature: Option<Vec<u8>>,
    pub raw_signatures: Vec<String>,
}

impl DidSignedMessage {

    pub fn parse(s: &str) -> Option<Self> {
        let mut raw_messages = vec![];
        let mut signed_headers = vec![];
        let mut raw_signatures = vec![];

        let mut is_in_message = false;
        let mut is_in_signed = false;
        let mut is_parse_header = true;
        let mut s_lines_iter = s.lines().peekable();
        while let Some(ln) = s_lines_iter.next() {
            if is_in_message { // parse message body
                if let Some(h) = extra_cmd_line(ln) {
                    if h == BEGIN_DID_SIGNATURE {
                        is_in_message = false;
                        is_in_signed = true;
                        continue;
                    }
                }
                raw_messages.push(ln.to_owned());
            } else if is_in_signed { // parse signature
                if let Some(h) = extra_cmd_line(ln) {
                    if h == END_DID_SIGNATURE {
                        break; // THE END!
                    }
                }
                if is_parse_header { // parse header
                    if ln.trim().is_empty() {
                        is_parse_header = false;
                        continue;
                    }
                    signed_headers.push(parse_header_line(ln, &mut s_lines_iter));
                } else { // parse signature body
                    raw_signatures.push(ln.to_owned());
                }
            } else if let Some(h) = extra_cmd_line(ln) {
                if h == BEGIN_DID_MESSAGE {
                    is_in_message = true;
                }
            }
        }

        let message = Some(vec![]); // TODO
        let signed_signature = decode_block_base64(&raw_signatures.join(""));

        Some(Self {
            message,
            raw_messages,
            signed_headers,
            signed_signature,
            raw_signatures,
        })
    }

    pub fn find_first_header(&self, key: &str) -> Option<String> {
        self.find_header(key).iter().next().map(|s| s.to_owned())
    }

    pub fn find_header(&self, key: &str) -> Vec<String> {
        self.signed_headers.iter()
            .filter(|h| h.key == key)
            .map(|h| h.value.clone())
            .collect::<Vec<_>>()
    }
}

fn parse_header_line(ln: &str, s_lines_iter: &mut std::iter::Peekable<std::str::Lines<'_>>) -> Header {
    let mut is_parse_header_key = true;
    let mut key = String::new();
    let mut val = String::new();
    for c in ln.chars() {
        if is_parse_header_key {
            if c == ':' {
                is_parse_header_key = false;
            } else {
                key.push(c);
            }
        } else {
            val.push(c);
        }
    }
    while let Some(next_ln) = s_lines_iter.peek() {
        if next_ln.starts_with("- ") {
            val.push('\n');
            val.push_str(&next_ln.chars().skip(2).collect::<String>());
            s_lines_iter.next();
        } else {
            break;
        }
    }
    Header { key: key.trim().to_owned(), value: val.trim().to_owned(), }
}

fn extra_cmd_line(ln: &str) -> Option<String> {
    if ln.starts_with(FIVE_MINUS) && ln.ends_with(FIVE_MINUS) {
        let mut all_c = vec![];
        for c in ln.chars() {
            all_c.push(c);
        }
        let mut head_minus_count = 0;
        for c in &all_c {
            if *c == '-' {
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
        for c in all_c.iter().take(all_c.len() - foot_minus_count).skip(head_minus_count) {
            let ignore_c = if let Some(last_c) = lastc {
                last_c == ' ' && *c == ' '
            } else {
                false
            };
            if !ignore_c {
                ret.push(*c);
            }
            lastc = Some(*c);
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
    assert_eq!(None, extra_cmd_line(" -----a-----"));
    assert_eq!(Some("a".to_owned()), extra_cmd_line("-----a-----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a b-----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a b -----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("-----a   b -----"));
    assert_eq!(Some("a b".to_owned()), extra_cmd_line("----- a   b -----"));
}