use std::collections::HashMap;
use std::io;

fn main() {
    let mut command = String::new();

    println!("Enter your command");

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    println!("Echo: {command}");

    let mut departments = HashMap::new();
    let mut accounting = vec!["Alice", "Bob"];
    let mut sales = vec!["John", "Rick"];

    departments.insert(String::from("Accounting"), accounting);
    departments.insert(String::from("Accounting"), accounting);

    // let score = departments.get("Accounting").copied().unwrap_or(0);
}
