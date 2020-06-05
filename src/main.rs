#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;

mod err;
mod did;

pub use err::*;
pub use did::*;

fn main() {
    println!("OpenDID cli!");
}
