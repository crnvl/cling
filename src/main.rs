use crate::api::get_debug;

mod api;
mod util;

fn main() {
    println!("{}", get_debug());
}
