#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let bruce = Student {
        name: "Bruce".to_owned(),
        locker: Some(32),
    };

    let peter = Student {
        name: "Peter".to_owned(),
        locker: None,
    };
    match bruce.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("{:?} dosent have a locker", bruce.name),
    }
    match peter.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("{:?} dosent have a locker", peter.name),
    }
}

