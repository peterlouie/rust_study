fn main() {
    let mut i = 3;
    loop {
        //why dosen't need to borrow
        println!("{:?}", i);
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            println!("{:?}", i);
            break;
        }
    }
    println!("done");

    let a = 3;

    println!("{:?}", a);
    println!("{:?}", a);
}
