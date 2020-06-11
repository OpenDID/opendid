#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;

mod err;
mod util;
mod did;
mod db;
mod did_resolver;
mod signed_message_parser;

pub use err::*;
pub use did::*;
pub use db::*;
pub use util::*;
pub use did_resolver::*;
pub use signed_message_parser::*;

fn main() {
    println!("OpenDID cli!");

    let m = DidSignedMessage::parse(r##"
-----BEGIN DID SIGNED MESSAGE-----
text message OR based64 message

hello world
-----BEGIN DID SIGNATURE------
DID: did:example:xxxxxxxxxxxxxxxxxxxxxx#key-1
Version: 0.0.1
Agent: OpenDID v0.0.0
Hash: SHA256
Comment: comments
- line 2 ....
- line 3 ....

YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh
YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh
YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh
YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW FhYWFh
YWFhYW FhYWFh YWFhYW FhYWFh YWFhYW E=
-----END DID SIGNATURE-----
    "##).unwrap();

    for ln in &m.raw_messages {
        println!("{}", ln);
    }
    println!("=========");
    for h in &m.signed_headers {
        println!("::: {} -> {}", h.key, h.value);
    }
    println!("---------");
    for ln in &m.raw_signatures {
        println!("{}", ln);
    }
    println!("---------");
    println!("{:?}", m.signed_signature);
    println!("{}", String::from_utf8_lossy(&m.signed_signature.clone().unwrap()));

    println!();
    println!();

    println!("{}", m.as_string());
}
