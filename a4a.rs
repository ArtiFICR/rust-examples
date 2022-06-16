// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn check_if_true(my_bool: bool) {
    match my_bool {
        true => println!("it's {:?}", my_bool),
        false => println!("it's {:?}", my_bool)
    }
}

fn main() {
    let my_bool = true;

    check_if_true(my_bool);
}
