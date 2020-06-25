// use crate::util::XResult;
use crate::util::{ encode_block_base64, decode_block_base64, };
use crate::err::DidSignedBuildError;

const SIGNED_MESSAGE_VERSION: &str = "0.0.1";

const NEW_LINE: &str = "\n";
const FIVE_MINUS: &str = "-----";
const BEGIN_DID_MESSAGE: &str = "BEGIN DID SIGNED MESSAGE";
const BEGIN_DID_SIGNATURE: &str = "BEGIN DID SIGNATURE";
const END_DID_SIGNATURE: &str = "END DID SIGNATURE";

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Base64,
    PlainText,
    Unknown,
}

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct DidSignedMessage {
    pub ty: MessageType,
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

        let type_header = find_last_header(&signed_headers, "type");
        let ty = match type_header {
            None => MessageType::PlainText, // default type is plain
            Some(header) => match header.value.to_ascii_lowercase().as_str() {
                "base64" => MessageType::Base64,
                "plain" | "text" => MessageType::PlainText,
                _ => MessageType::Unknown,
            },
        };
        let message = match ty {
            MessageType::PlainText => Some(raw_messages.join("\n").as_bytes().to_vec()),
            MessageType::Base64 => decode_block_base64(&raw_messages.join("")),
            MessageType::Unknown => None,
        };

        let signed_signature = decode_block_base64(&raw_signatures.join(""));

        Some(Self {
            ty,
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

    pub fn as_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(FIVE_MINUS);
        ret.push_str(BEGIN_DID_MESSAGE);
        ret.push_str(FIVE_MINUS);
        ret.push_str(NEW_LINE);

        for m in &self.raw_messages {
            ret.push_str(m);
            ret.push_str(NEW_LINE);
        }

        ret.push_str(FIVE_MINUS);
        ret.push_str(BEGIN_DID_SIGNATURE);
        ret.push_str(FIVE_MINUS);
        ret.push_str(NEW_LINE);

        for h in &self.signed_headers {
            ret.push_str(&h.key);
            ret.push_str(": ");
            ret.push_str(&h.value.split('\n').collect::<Vec<_>>().join("\n- "));
            ret.push_str(NEW_LINE);
        }
        ret.push_str(NEW_LINE);
        if let Some(signed_signature) = &self.signed_signature {
            ret.push_str(&encode_block_base64(&signed_signature[..], 0, 0));
            ret.push_str(NEW_LINE);
        }

        ret.push_str(FIVE_MINUS);
        ret.push_str(END_DID_SIGNATURE);
        ret.push_str(FIVE_MINUS);
        ret.push_str(NEW_LINE);

        ret
    }
}

#[derive(Debug)]
pub struct DidSignedMessageBuilder {
    ty: MessageType,
    message: Option<Vec<u8>>,
    raw_messages: Option<String>,
    signed_headers: Vec<Header>,
    signature: Option<Vec<u8>>,
}

impl DidSignedMessageBuilder {

    pub fn new_from_bytes(m: &[u8]) -> Self {
        let s = DidSignedMessageBuilder{
            ty: MessageType::Base64,
            message: Some(m.to_vec()),
            raw_messages: None,
            signed_headers: vec![],
            signature: None,
        };
        s.default_init()
    }

    pub fn new_from_str(m: &str) -> Self {
        let s = DidSignedMessageBuilder{
            ty: MessageType::PlainText,
            message: None,
            raw_messages: Some(m.to_owned()),
            signed_headers: vec![],
            signature: None,
        };
        s.default_init()
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.signed_headers.push(Header{ key: key.into(), value: value.into(), });
        self
    }

    pub fn key_id(self, did: &str) -> Self {
        self.header("KeyId", did)
    }

    pub fn comment(self, comments: &str) -> Self {
        self.header("Comment", comments)
    }

    pub fn hash(self, hash: &str) -> Self {
        self.header("Hash", hash)
    }

    pub fn signature(mut self, sign: &[u8]) -> Self {
        self.signature = Some(sign.to_vec());
        self
    }

    pub fn build(self) -> Result<DidSignedMessage, DidSignedBuildError> {
        let mut has_did = false;
        for header in &self.signed_headers {
            if header.key == "KeyId" {
                has_did = true;
            }
        }
        if !has_did {
            return Err(DidSignedBuildError::HeaderKeyIdMissError);
        }
        let msg = match self.ty {
            MessageType::Base64 => self.message.map(|m| encode_block_base64(&m, 0, 0)).unwrap_or_else(|| "".to_owned()),
            MessageType::PlainText => self.raw_messages.unwrap_or_else(|| "".to_owned()),
            MessageType::Unknown => "!!!ERROR: Type Unknown!".to_owned(),
        };
        let raw_messages = msg.lines().map(|ln| ln.to_owned()).collect::<Vec<_>>();
        let signed_signature = match self.signature {
            None => return Err(DidSignedBuildError::SignatureMissError),
            Some(sign) => sign,
        };
        Ok(DidSignedMessage {
            ty: self.ty,
            message: None,
            raw_messages,
            signed_headers: self.signed_headers,
            signed_signature: Some(signed_signature),
            raw_signatures: vec![],
        })
    }

    fn default_init(self) -> Self {
        self.header("Version", SIGNED_MESSAGE_VERSION)
            .header("Agent", &format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
    }
}

fn find_last_header<'a>(signed_headers: &'a [Header], header: & str) -> Option<&'a Header> {
    let mut ret = None;
    let header_ascii_lowercase = header.to_ascii_lowercase();
    for h in signed_headers {
        if h.key.to_ascii_lowercase() == header_ascii_lowercase {
            ret = Some(h);
        }
    }
    ret
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