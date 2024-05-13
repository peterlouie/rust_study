fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    //for loop with a match
    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("number of elements = {:?}", my_numbers.len())
}
