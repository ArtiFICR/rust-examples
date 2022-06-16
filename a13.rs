// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers -- DONE
// * Iterate through the vector using a for..in loop -- DONE
// * Determine whether to print the number or print "thirty" inside the loop -- DONE
// * Use the .len() function to print the number of elements in a vector -- DONE

fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("# of elements: {:?}", my_numbers.len());
}
