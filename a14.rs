// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color --DONE
// * The color and name should be stored as a String --DONE
// * Create and store at least 3 people in a vector --DONE
// * Iterate through the vector using a for..in loop --DONE
// * Use an if expression to determine which person's info should be printed --DONE
// * The name and colors should be printed using a function --DONE

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let group_of_people = vec![
        Person {
            name: String::from("Jack"),
            age: 6,
            favorite_color: String::from("green"),
        },
        Person {
            name: String::from("Sal"),
            age: 22,
            favorite_color: String::from("blue"),
        },
        Person {
            name: String::from("Alice"),
            age: 11,
            favorite_color: String::from("red"),
        },
        Person {
            name: String::from("Bobby"),
            age: 4,
            favorite_color: String::from("purple"),
        },
        Person {
            name: String::from("Timmy"),
            age: 8,
            favorite_color: String::from("yellow"),
        },
    ];

    println!("-------------------");

    for person in group_of_people {
        if person.age < 11 {
            print(&person.name);
            println!("age: {:?}", person.age);
            print(&person.favorite_color);
            println!("-------------------");
        }
    }
}
