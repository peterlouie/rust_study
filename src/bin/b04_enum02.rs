enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    print_color(Color::Yellow)
}
