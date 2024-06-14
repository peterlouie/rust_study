//survey program
#[derive(Debug)]
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("answer q1: {:?}", ans),
        None => println!("answer q1: no response"),
    }

    match response.q2 {
        Some(ans) => println!("answer q2: {:?}", ans),
        None => println!("answer q2: no response"),
    }

    match response.q3 {
        Some(ans) => println!("answer q3: {:?}", ans),
        None => println!("answer q3: no response"),
    }
}

