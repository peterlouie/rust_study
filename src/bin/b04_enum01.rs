enum Direction {
    Left,
    _Right,
}

fn main() {
    let go = Direction::Left;

    match go {
        Direction::Left => println!("go left"),
        Direction::_Right => println!("go right"),
    }
}
