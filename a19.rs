// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();

    store.insert("Chair", 5);
    store.insert("Bed", 3);
    store.insert("Table", 2);
    store.insert("Couch", 0);

    let mut stock_count = 0;

    for (item, qty) in store.iter() {
        match qty {
            0 => println!("furniture: {:?}, # in stock: out of stock", item),
            _ => {
                println!("furniture: {:?}, # in stock: {:?}", item, qty);
                stock_count += qty;
            },
        }
    }

    println!("Total stock: {:?}", stock_count);
}
