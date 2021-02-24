#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
extern crate rocket;
extern crate ws;
mod book;
mod client;
mod order;
mod server;

use book::Book;
use client::Client;
use server::Server;
use ws::{
    connect, listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result,
    Sender,
};

fn main() {
    // thread::spawn(move || {
    //     println!("New Thread");
    //     connect("wss://ws-feed.pro.coinbase.com", |out| Client { out: out }).unwrap();
    // });
    Client::new();

    // let MainBook =  Book::new();
    // println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // println!("HAHHHHHHHHHHHHHHH{:?}",MainBook)
    // assert_eq!(rx.recv().unwrap(), 10);
    // rocket::ignite().mount("/", routes![index]).launch();
}
