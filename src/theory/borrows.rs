use std::io;

fn main() {
    let mut accounting = vec!["Alice", "Ben"];
    
    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();        
                            
        // add_vec is a vec containing references to slices of strings that live on the stack.
        // Those only live between the brackets declared as a loop statement.
        // You then want to add these references to string slices to a vec that is created outside of the loop.
        // The problem is that when the loop exists, the string slices will get dropped, and the vec outside will have dangling references.
        // That is not allowed. By changing it to strings instead, they are now stored on the heap, and will not get dropped when we exit the loop.

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);
    }
}