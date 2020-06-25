#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prettytable;

mod err;
mod util;
mod p256k1;
mod did;
mod db;
mod storage;
mod cmd;
mod did_resolver;
mod algo;
mod signed_message_parser;
mod cmd_default;
mod cmd_did_document;
mod cmd_test;

use clap::{ App, Arg, };

pub use err::*;
pub use util::*;
pub use p256k1::*;
pub use did::*;
pub use db::*;
pub use storage::*;
pub use cmd::*;
pub use did_resolver::*;
pub use signed_message_parser::*;
use cmd_default::*;
use cmd_did_document::*;
use cmd_test::*;

#[tokio::main]
async fn main() -> XResult<()> {
    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(CommandTest{}),
        Box::new(CommandDidCocument{}),
    ];
    let mut app = App::new("OpenDID")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("A DID command line tool")
                    .arg(
                        Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .multiple(true)
                        .help("Show verbose info")
                    );
    for command in &commands {
        if let Some(subcommand) = command.subcommand() {
            app = app.subcommand(subcommand);
        }
    }
    let matches = app.get_matches();

    for command in &commands {
        if let Some(sub_cmd_matches) = matches.subcommand_matches(command.name()) {
            return command.run(&matches, sub_cmd_matches);
        }
    }

    // let did = "did:ccp:3nBPSZU1q6mmxha5Jbg8NcRNGGNt";
    // let did_document = did_resolver::DidResolver::new_baidu().resolve(did).await?;
    // let s = storage::Storage::new_default()?;
    // s.set_did_document(did, &did_document)?;

    CommandDefault{}.run(&matches, &matches)
}
