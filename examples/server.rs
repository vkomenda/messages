#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate messages;

use messages::*;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let addr = format!("{}:{}", "127.0.0.1", "12000").parse().unwrap();
    start_server(&addr);
}
