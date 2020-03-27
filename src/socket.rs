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

pub struct Socket {
}

impl Socket {
    pub fn init(wss:&str) {
        // Connect to the url and call the closure
        if let Err(error) = connect(wss, |out| {
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
                   let incoming_message:json = json::parse(&text).unwrap();
                    println!("Client got message '{:?}'. ", incoming_message["type"].to_string());
                }
                Ok(())
            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    }
}


// fn parse_data(input:&str){
//     json::parse(input)
// }