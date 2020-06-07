#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;

mod err;
mod util;
mod did;
mod db;

pub use err::*;
pub use did::*;
pub use db::*;

fn main() {
    println!("OpenDID cli!");
}
