// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Vanilla,
    Cola,
    Cherry
}

struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}

fn print_drink_info(drink: Drink) {
    println!("Drink info:");
    match drink.flavor {
        Flavor::Orange => println!("Flavor: Orange"),
        Flavor::Vanilla => println!("Flavor: Vanilla"),
        Flavor::Cola => println!("Flavor: Cola"),
        Flavor::Cherry => println!("Flavor: Cherry"),
    }

    println!("Ounces: {:?}\n", drink.fluid_ounces);
}

fn main() {
    let orange = Drink {
        flavor: Flavor::Orange,
        fluid_ounces: 12.3,
    };
    print_drink_info(orange);

    let vanilla = Drink {
        flavor: Flavor::Vanilla,
        fluid_ounces: 10.2,
    };
    print_drink_info(vanilla);

    let cola = Drink {
        flavor: Flavor::Cola,
        fluid_ounces: 8.0,
    };
    print_drink_info(cola);

    let cherry = Drink {
        flavor: Flavor::Cherry,
        fluid_ounces: 6.5,
    };
    print_drink_info(cherry);
}
