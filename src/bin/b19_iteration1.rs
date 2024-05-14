fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers
        //collect will make new vector
        .iter()
        .map(|num| num + 1)
        .collect();
    dbg!(plus_one);
}
