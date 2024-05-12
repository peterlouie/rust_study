#[derive(Debug)]
struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_quatity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let apple = GroceryItem {
        id: 123,
        quantity: 3,
    };

    print_quatity(&apple);
    print_id(&apple)
}
