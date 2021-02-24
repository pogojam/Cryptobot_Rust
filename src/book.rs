#![allow(dead_code)]
/*

BOOK

Methods:
    Add: Adds trade to book.
    remove: Removes trade from book.
    get: Gets trade from book.

Struct:
    : Holds trades
*/

use crate::order::Order;
use std::collections::HashMap;
use std::thread;
// use fnv::FnvHashMap;

#[derive(Debug)]
pub struct Trade {
    Price: i32,
}

#[derive(Debug)]
pub struct Book {
    Bids: HashMap<i32, Trade>,
    Asks: HashMap<i32, Trade>,
}

impl Book {
    pub fn new() -> Book {
        // thread::spawn(move || {
        //     println!("New Book Created");
        // });

        Book {
            Bids: HashMap::new(),
            Asks: HashMap::new(),
        }
    }
    // pub fn add (&mut self,order:Order){
    //         self.Bids
    // }
    // thread::spawn(|| Book::new());
}
