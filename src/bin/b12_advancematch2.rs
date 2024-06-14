//#[derive(Debug)]
enum Ticket {
    //better to be alphabetic order
    //f64 first because all variant have it
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}


fn main() {
    let tickets = vec![
        Ticket::Backstage (65.0, "Bruce".to_owned()),
        Ticket::Standard(25.0),
        Ticket::Vip(65.0, "Peter".to_owned()),
    ];

    //println!("{:?}", Ticket::Standard(0.0));

    for ticket in tickets{
        match ticket {
            //{} when need more to write
            Ticket::Backstage(price, holder) => {
                println!("Backstage price: {:?}, holder: {:?}", price, holder)
            },
            //drop variable price and holder
            Ticket::Vip(_price, _holder) => println!("Vip"),
            Ticket::Standard(price) => println!("{:?}", price)
            //_ => (),
        }
    }
}
