enum Status {
    Error(i32),
    Info,
    Warn,
}

fn main() {
    let status = Status::Error(5);
    match status {
        Status::Error(s @ 3) => println!("error three"),
        Status::Error(s @ 5..=6) => println!("error 5 or 6: {}", c),
        Status::Error(s @ 4..=10) => println!("error three through ten: {}", c),
        Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
        Status::Error(s) => println!("error code: {}", c),
        Status::Info => println!("info"),
        Status::Warn => println!("warn"),
    }
}