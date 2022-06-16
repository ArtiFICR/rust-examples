// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("Quantity: {:?}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("ID: {:?}", item.id);
}

fn main() {
    let item = GroceryItem {
        quantity: 17,
        id: 4738
    };

    print_quantity(&item);
    print_id(&item);
}
