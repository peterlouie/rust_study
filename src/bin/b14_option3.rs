enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    try_access(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Position::Manager,
        status: Status::Terminated,
    };

    //using match with a function
    match print_access(&manager) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
