fn main() {
    let smiley = "\u{1f642}"; // Sets unicode value for smiley emoji to a variable
    println!("{:?}", smiley); // Prints a smiley emoji to terminal

    // Raw string example
    let msg = r#"Over "there""#;
    println!("{:?}", msg);

    let msg2 = r"
    left
    right";
    println!("{:?}", msg2);

    let msg3 = r##"Over #"#there#"#"##;
    println!("{:?}", msg3);
}