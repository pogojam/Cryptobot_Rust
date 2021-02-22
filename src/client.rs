extern crate ws;
extern crate url;
use url::Url;
use ws::{connect,Handler, Sender, Handshake, Request,Result,Response, Message, CloseCode,Error,listen};

const Connection_Request:&str= r#"
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


pub struct Client {
    pub out: Sender,
}

impl Handler for Client {

    fn on_open(&mut self, _: Handshake) -> Result<()> {



        println!("Socket Opened {}",Connection_Request);
        self.out.send(Connection_Request)
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("Got message: {}", msg);
        self.out.close(CloseCode::Normal)
    }
    fn on_error(&mut self,err: Error){
        println!("Got message: {}",err);
    }
    fn on_close(&mut self,code: CloseCode, reason: &str){
        println!("Closing Socket");
    }
    fn build_request(&mut self, url: &Url)-> Result<Request>{
        println!("Request URL {}",url);
        let mut req = Request::from_url(url)?;
        req.add_extension("permessage-deflate; client_max_window_bits");
        Ok(req)
    }
}

