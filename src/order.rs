enum Type () {
    open,
    match,
    posted,
    done,
    l2update
}


enum Order {
        type:<Type>
}

impl Order {
        pub fn new() -> Order{
            Order {
                type:Type::open
            }
        }
}