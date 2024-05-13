#[derive(Debug)]
struct Customer {
    age: i32,
}

fn try_purchase(customer: Customer) -> Result<(), String> {
    if customer.age < 21 {
        Err("need to be 21".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let bruce = Customer { age: 20 };
    println!("{:?}", try_purchase(bruce))
}

