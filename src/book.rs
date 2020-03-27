
enum Side {
    bid,
    ask
}

struct Order{
    Type:String,
    Price:i32,
    Side:Side
}

// struct Book  {
//     Bids:[Orders::Bid],
//     Sells:[Orders::Ask]
// }

pub fn add (){
    let mut book = vec![];
        book.push(Order{
            Type:"test".to_string(),
            Price:100,
            Side:Side::bid
        })

}