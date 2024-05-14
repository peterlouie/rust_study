mod greet {
    pub fn hello() {
        println!("hello");
    }

    pub fn goodbye() {
        println!("goodbye");
    }
}

fn main() {
    use greet::hello;

    hello();
}
