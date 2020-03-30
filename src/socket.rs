extern crate ws;
extern crate json;
use ws::{connect, CloseCode,Message};

const CONFIG:&str = r#"
{
    "type": "subscribe",
    "product_ids": [
        "BTC-USD"
    ],
    "channels": [
        "level2",
        "full"
    ]
}
"#;

pub struct Socket <'a>  
{
    wss: &'a str,
   pub controller:fn()->()
}

impl <'a> Socket <'a>{

    pub fn new(wss:&'a str , controller:fn()->())->Self{
            Socket{
                wss,
                controller
            }
    }

    pub fn init(&self){
        // Connect to the url and call the closure
        if let Err(error) = connect(self.wss, |out| {
            // Queue a message to be sent when the WebSocket is open
            if out.send(CONFIG).is_err() {
                println!("Websocket couldn't queue an initial message.")
            } else {
                println!("Client sent message 'Hello WebSocket'. ")
            }
            // The handler needs to take ownership of out, so we use move
            move |msg:Message| {
                //If there is Text of Ok(type:text) , set text = msg.into_text()
                if let Ok(mut text) = msg.into_text() {
                    // let order = Order::new(json::parse(&text).unwrap());
                   let incoming_message = json::parse(&text).unwrap();
                    // Socket::on_message(10);
                    println!("Client got message '{:?}'. ", incoming_message["type"].to_string());
                }
                Ok(())
            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    }

    pub fn set_controller (){}
    
    // pub fn on_message(callback:Fn(&i32))-> impl Fn(i32)  {
    //        move |msg| callback(&msg)
    // }
}


// fn parse_data(input:&str){
//     json::parse(input)
// }