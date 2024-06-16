struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("check in as pilot")
    }
    fn process(&self) {
        println!("Pilot enter the cockpit");
    }
}

struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("check in as passenger")
    }
    fn process(&self) {
        println!("passenger take a seat");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("check in as cargo")
    }
    fn process(&self) {
        println!("cargo move to storage");
    }
}

trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let paul = Passenger;
    let kathy = Pilot;
    let cargo1 = Cargo;
    let cargo2 = Cargo;
    process_item(paul);
    process_item(kathy);
    process_item(cargo1);
    process_item(cargo2)
}
