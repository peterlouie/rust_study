fn main() {
    let coord = (2, 3);
    println!("{:?} {:?}", coord.0, coord.1);

    //destructure
    let (x, y) = (2, 3);
    println!("{:?} {:?}", x, y);

    //bad example
    let favorites = ("red", 14, "pizza");
    let color = favorites.0;
    let age = favorites.1;
    let food = favorites.2;

    println!("{:?} {:?} {:?}", color, age, food)
}

