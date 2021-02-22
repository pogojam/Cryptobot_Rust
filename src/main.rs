#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]
#[macro_use]
extern crate rocket;
extern crate ws;
mod book;
mod client;
mod order;
mod server;

use client::Client;
use std::thread;
use std::sync::mpsc::channel;
use book::Book;
use server::Server;
use url::Url;
use ws::{connect,Handler, Sender, Handshake, Request,Result,Response, Message, CloseCode,Error,listen};



fn main() {
    // let (tx,rx) = channel();
        connect("wss://ws-feed.pro.coinbase.com", |out| { Client { out: out } }).unwrap();

    // thread::spawn(move || {
    //     listen("127.0.0.1:3012", |out| { Server { out: out } }).unwrap();
    // });
    // thread::spawn(move || {
    // });

    // let MainBook =  Book::new();
    // println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // println!("HAHHHHHHHHHHHHHHH{:?}",MainBook)
    // assert_eq!(rx.recv().unwrap(), 10);
    // rocket::ignite().mount("/", routes![index]).launch();
}
