struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print_data(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Peter"),
            fav_color: String::from("red"),
            age: 7,
        },
        Person {
            name: String::from("Bruce"),
            fav_color: String::from("yellowj"),
            age: 9,
        },
        Person {
            name: String::from("Ava"),
            fav_color: String::from("pink"),
            age: 11,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print_data(&person.name);
            print_data(&person.fav_color);
        }
    }
}
