// enum Type () {
//     open,
//     match,
//     posted,
//     done,
//     l2update
// }


pub struct Order {
        r#type:String,
        price:String
}

impl Order {
        pub fn new() -> Order{
            Order {
                r#type:"open".to_string(),
                price:"10.3".to_string()
            }
        }
}