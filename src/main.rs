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
pub use did_resolver::*;
pub use signed_message_parser::*;

fn main() {
    println!("OpenDID cli!");
}
