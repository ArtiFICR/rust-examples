// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn loop_integer_to_four() {
    let mut my_integer = 1;

    loop {
        println!("{:?}", my_integer);
        if my_integer == 4 {
            break;
        }
        my_integer += 1;
    }
}

fn main() {
    loop_integer_to_four();
}
