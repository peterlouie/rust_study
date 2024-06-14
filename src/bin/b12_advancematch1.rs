enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        //renaming it to other for readable code
        //instead of puting _
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(25);

    match flat {
        Discount::Flat(2) => println!("flat discount 2"),
        Discount::Flat(amount) => println!("flat discount {:?}", amount),
        //this means ignore everything Discount::Percent
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 25,
    };

    match concert {
        //with price value and just printing the event
        Ticket { price: 25, event } => println!("price @ 50 event = {:?}", event),
        // .. or 2 dots is saying the price is the only concern
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
