#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code,u)]
#[macro_use]
extern crate rocket;
extern crate ws;
mod book;
mod socket;
mod order;


use std::thread;
use std::sync::mpsc::channel;
use socket::Socket;
use book::Book;




fn main() {
    // let (tx,rx) = channel();
    thread::spawn(move || {
        let CoinBase_Socket = Socket::new("wss://ws-feed.pro.coinbase.com",||println!("hi"));
        (CoinBase_Socket.controller)();
        CoinBase_Socket.init();
    });
    let MainBook =  Book::new();
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("HAHHHHHHHHHHHHHHH{:?}",MainBook)
    // assert_eq!(rx.recv().unwrap(), 10);
    // rocket::ignite().mount("/", routes![index]).launch();
}
