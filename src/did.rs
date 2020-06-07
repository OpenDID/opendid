
use crate::err::DidError;
use std::collections::HashMap;

// DID method registry: https://w3c-ccg.github.io/did-method-registry/
lazy_static! {
    static ref DID_METHOD_HASHMAP: HashMap<&'static str, DidMethod> = {
        let methods = vec![
            DidMethod::new("abt", DidStatus::PROVISIONAL, "ABT Network",
                            ["ArcBlock"].to_vec(),
                            "ABT DID Method", "https://arcblock.github.io/abt-did-spec/"),
            DidMethod::new("btcr", DidStatus::PROVISIONAL, "Bitcoin",
                            ["Christopher Allen", "Ryan Grant", "Kim Hamilton Duffy"].to_vec(),
                            "BTCR DID Method", "https://w3c-ccg.github.io/didm-btcr"),
            DidMethod::new("stack", DidStatus::PROVISIONAL, "Bitcoin",
                            ["Jude Nelson"].to_vec(),
                            "Blockstack DID Method", "https://github.com/blockstack/blockstack-core/blob/stacks-1.0/docs/blockstack-did-spec.md"),
            // ...
            DidMethod::new("example", DidStatus::PROVISIONAL, "DID Specification",
                            ["W3C Credentials Community Group"].to_vec(),
                            "DID Specification", "https://w3c-ccg.github.io/did-spec/"),
            // ...
            DidMethod::new("ccp", DidStatus::PROVISIONAL, "Quorum",
                            ["Baidu, Inc."].to_vec(),
                            "Cloud DID Method", "https://did.baidu.com/did-spec/"),
        ];
        let mut m = HashMap::new();
        for method in methods {
            m.insert(method.method, method);
        }
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub enum DidStatus {
    PROVISIONAL,
    DEPRECATED,
}

#[derive(Debug, Clone)]
pub struct DidMethod {
    pub method: &'static str,
    pub status: DidStatus,
    pub dlt_or_network: &'static str,
    pub authors: Vec<&'static str>,
    pub link_text: &'static str,
    pub link: &'static str,
}

impl DidMethod {
    pub fn new(method: &'static str, status: DidStatus, dlt_or_network: &'static str, authors: Vec<&'static str>,
        link_text: &'static str, link: &'static str) -> Self {
        DidMethod {
            method, status, dlt_or_network, authors, link_text, link,
        }
    }
}

// did                = "did:" method-name ":" method-specific-id
// method-name        = 1*method-char
// method-char        = %x61-7A / DIGIT
// method-specific-id = *( ":" *idchar ) 1*idchar
// idchar             = ALPHA / DIGIT / "." / "-" / "_"
#[derive(Debug, Clone)]
pub struct Did {
    pub method: String,
    pub id: String,
}

impl Did {

    pub fn new(method: &str, id: &str) -> Result<Self, DidError> {
        let did_method = DID_METHOD_HASHMAP.get(method);
        if did_method.is_none() {
            return Err(DidError::FormatError(format!("DID method not found: did:{}:{}", method, id)));
        }
        Ok(Did {
            method: method.into(),
            id: id.into(),
        })
    }

    pub fn parse(did: &str) -> Result<Self, DidError> {
        if !did.starts_with("did:") {
            return Err(DidError::FormatError(format!("DID not starts with: 'did:', {}", did)));
        }
        let method_and_addr = did.chars().skip(4).collect::<String>();

        let mut method = String::new();
        let mut id = String::new();
        let mut has_c = false;
        for c in method_and_addr.chars() {
            if has_c {
                id.push(c);
            } else if c == ':' {
                has_c = true;
            } else {
                method.push(c);
            }
        }
        Self::new(&method, &id)
    }
    
    pub fn to_string(&self) -> String {
        format!("did:{}:{}", self.method, self.id)
    }
}

// did-url            = did path-abempty [ "?" did-query ]
//                      [ "#" fragment ]
// did-query          = param *( "&" param )
// param              = param-name "=" param-value
// param-name         = 1*pchar
// param-value        = *pchar
#[derive(Debug, Clone)]
pub struct DidKey { // TODO ...
    pub did: Did,
    pub key: String,
}

impl DidKey {

    pub fn new(did: Did, key: &str) -> Self {
        DidKey { did, key: key.into(), }
    }

    pub fn parse(did_key: &str) -> Result<Self, DidError> {
        let did_and_key = did_key.split('#').collect::<Vec<_>>();
        if did_and_key.len() != 2 {
            return Err(DidError::FormatError(format!("DIDKey format error: {}", did_key)));
        }
        let did = Did::parse(did_and_key[0])?;
        let key = did_and_key[1];
        Ok(Self::new(did, key))
    }

    pub fn to_string(&self) -> String {
        format!("{}#{}", self.did.to_string(), self.key)
    }
}

#[test]
fn test_did_new() {
    assert_eq!("did:example:test_addr".to_owned(), Did::new("example", "test_addr").unwrap().to_string());
    assert_eq!(true, Did::new("example_not_exists", "test_addr").is_err());
}

#[test]
fn test_did_parse() {
    assert_eq!("did:example:test_addr".to_owned(), Did::parse("did:example:test_addr").unwrap().to_string());
    assert_eq!(true, Did::parse("did:example_not_exists:test_addr").is_err());
    assert_eq!(true, Did::parse("ddd:example_not_exists:test_addr").is_err());
}
