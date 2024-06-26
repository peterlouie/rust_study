struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    //fn show_temp(temp: Temperature) {
    //&self meaning already have a variable
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }

    //Self meaning no variable yet
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    //Temperature::show_temp(&hot);
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
    //test
}
