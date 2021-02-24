extern crate url;
extern crate ws;
use std::sync::mpsc::{channel, Receiver, Sender as ChannelSender};
use std::thread;
use url::Url;
use ws::{
    connect, listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result,
    Sender,
};

const Connection_Request: &str = r#"
{
    "type": "subscribe",
    "product_ids": [
        "ETH-USD",
        "ETH-EUR"
    ],
    "channels": [
        "level2",
        "heartbeat",
        {
            "name": "ticker",
            "product_ids": [
                "ETH-BTC",
                "ETH-USD"
            ]
        }
    ]
}
"#;

pub struct Client<T> {
    pub out: Sender,
    pub channel_out: ChannelSender<T>,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Socket Opened {}", Connection_Request);
        self.out.send(Connection_Request)
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("Got message: {}", msg);
        self.out.close(CloseCode::Normal)
    }
    fn on_error(&mut self, err: Error) {
        println!("Got message: {}", err);
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Closing Socket");
    }
    fn build_request(&mut self, url: &Url) -> Result<Request> {
        println!("Request URL {}", url);
        let mut req = Request::from_url(url)?;
        req.add_extension("permessage-deflate; client_max_window_bits");
        Ok(req)
    }
}

impl<T> Client<T> {
    pub fn new() -> Client {
        let (tx, rx) = channel();
        thread::spawn(move || {
            println!("New Thread");
            connect("wss://ws-feed.pro.coinbase.com", |out| Client {
                out: out,
                channel_out: tx,
            })
            .unwrap();
        });
    }
}
