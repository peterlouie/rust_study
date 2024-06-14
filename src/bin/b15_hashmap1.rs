use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "barbie".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "shoes".to_owned(),
        },
    );

    //dosen't work
    println!("{:?}", lockers);

    for (key, value) in lockers.iter() {
        println!("{:?} {:?}", key, value)
    }
}
