use crate::order::Order;

pub struct Book {
    Bids:Vec<Order>,
    Sells:Vec<Order>
}

impl Book {
    pub fn new ()->Book{
        Book{
            Bids:vec![],
            Sells:vec![]
        }
    }
    pub fn add (&mut self,order:Order){
            self.Bids.push(order)
    }
}