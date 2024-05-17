use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();

    let local: DateTime<Local> = Local::now();

    println!("{} local time", local);
    println!("{} utc time", utc);

    print!("{:?}", local.format("%Y-%m-%d %H:%M:%S").to_string())
}

