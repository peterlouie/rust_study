fn main() {
    let some_bool = true;

    match some_bool {
        true => println!("it's true"),
        false => println!("its false"),
    }

    let some_int = 3;

    match some_int {
        3 => println!("it is 3"),
        _ => println!("it is unknown"),
    }
}
