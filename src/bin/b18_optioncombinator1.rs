fn main() {
    let a: Option<i32> = Some(1);

    let a_filtered = a.filter(|num| *num == 2);
    //output none since a = 1 num = 2
    println!("{:?}", a_filtered);
    //if a have data nothing happen
    let a_or_else = a.or_else(|| Some(6));
    dbg!(a_or_else);
    //make a default when data is not present
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}

