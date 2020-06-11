#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;

mod err;
mod util;
mod did;
mod db;
mod cmd;
mod did_resolver;
mod signed_message_parser;
mod cmd_default;
mod cmd_test;

use clap::{ App, Arg, };

pub use err::*;
pub use util::*;
pub use did::*;
pub use db::*;
pub use cmd::*;
pub use did_resolver::*;
pub use signed_message_parser::*;
use cmd_default::*;
use cmd_test::*;

fn main() -> XResult<()> {
    let commands = vec![
        CommandTest{},    
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

    CommandDefault{}.run(&matches, &matches)
}
