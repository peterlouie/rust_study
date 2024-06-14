fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let data: Vec<_> = data
        .iter()
        .map(|num| num * 3)
        .filter(|num| *num > 10)
        .collect();

    for num in data {
        println!("{:?}, ", num)
    }
}
