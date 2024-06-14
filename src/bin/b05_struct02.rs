enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: Sparkling"),
        Flavor::Sweet => println!("flavor: Sweet"),
        Flavor::Fruity => println!("flavor: Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 8.0,
    };

    print_drink(sweet)
}
