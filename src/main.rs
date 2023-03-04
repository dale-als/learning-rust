use std::io;

fn main() {
    let mut accounting = vec!["Alice", "Ben"];
    
    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);
    }
}