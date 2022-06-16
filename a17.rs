// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let some_string = "I aM a WeIrd stRiNg";

    println!("Normal string: {:?}", some_string);
    println!("Lowercase string: {:?}", some_string.to_lowercase());
    println!("Uppercase string: {:?}", some_string.to_uppercase());
}
