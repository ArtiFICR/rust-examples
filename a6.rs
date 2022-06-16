// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn countdown_from_five() {
    let mut counter = 5;
    println!("Countdown:");
    while counter >= 1 {
        println!("{:?}", counter);
        counter -= 1;
    }
    println!("done!");
}

fn main() {
    countdown_from_five();
}
