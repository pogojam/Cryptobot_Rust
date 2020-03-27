#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate ws;
mod book;
mod socket;
use std::thread;
use socket::Socket;

macro_rules! testmac {
    ( $( $i:expr ),* ) => {
        $(
            println!("Hi {}",String::from($i))
        );*
    };
}

// fn testmac(input: String) {
//     println!("{}", input);
// }
const Coinbase_wss: &str = "wss://ws-feed.pro.coinbase.com";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    book::add();
    thread::spawn(|| Socket::init(Coinbase_wss));
    rocket::ignite().mount("/", routes![index]).launch();
}
