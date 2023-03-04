use std::collections::HashMap;
use std::io;

fn main() {
    let accounting_name = "Accounting";
    let sales_name = "Sales";
    let mut accounting = vec![String::from("Alice"), String::from("Bob")];
    // let mut accounting = vec!["Alice", "Bob"];
    let mut sales = vec![String::from("Ben"), String::from("John")];
    // let mut sales = vec!["Ben", "John"];
    let mut departments = HashMap::new();
    let possible_commands = ["Add", "List"];

    departments.insert(accounting_name, accounting);
    departments.insert(sales_name, sales);

    'command: loop {
        greet(&departments, possible_commands);
        println!("Please enter your command or type Quit to quit:");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = &command.trim()[..];

        if command == "Quit" {
            break;
        }

        if !possible_commands.contains(&command) {
            println!("Incorrect command");
            continue;
        }

        match command {
            "Add" => {
                println!("-------------");
                println!("Enter target department and person separated by whitespace");
                println!("Example: Accounting John");
                println!("-------------");

                loop {
                    let mut add_input = String::from("");

                    io::stdin()
                        .read_line(&mut add_input)
                        .expect("Failed to read line");

                    let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

                    if add_vec.len() < 2 {
                        println!("Incorrect input, try again");
                        continue;
                    }

                    let target = add_vec[0];
                    let person = add_vec[1];

                    let department = departments.get_mut(target);

                    match department {
                        Some(dep_ref) => {
                            println!("-------------");
                            println!("Adding {person} to {target}");
                            dep_ref.push(person.to_string());
                            continue 'command;
                        }
                        None => {
                            println!("-------------");
                            println!("No such departnment");
                            continue 'command;
                        }
                    }
                }
            }
            "List" => {
                println!("-------------");
                println!("Enter target department");
                println!("Example: Accounting");
                println!("-------------");

                loop {
                    let mut list_input = String::from("");

                    io::stdin()
                        .read_line(&mut list_input)
                        .expect("Failed to read line");

                    let target = list_input.trim();
                    let department = departments.get_mut(target);

                    match department {
                        Some(dep_ref) => {
                            println!("-------------");
                            println!("Listing {target}:");
                            dep_ref.sort();
                            for person in dep_ref {
                                println!("{person}");
                            }
                            continue 'command;
                        }
                        None => {
                            println!("-------------");
                            println!("No such departnment");
                            continue 'command;
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn greet(departments: &HashMap<&str, Vec<String>>, commands: [&str; 2]) {
    println!("-------------");
    println!("There are two departments:");

    for (name, _people) in departments {
        println!("  - {name}");
    }

    println!("\n");
    println!("We support two commands:");

    for command in commands {
        println!("  - {command}");
    }
    println!("-------------");
}
