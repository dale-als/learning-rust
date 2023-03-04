use std::collections::HashMap;
use std::{io, vec};

fn main() {
    let accounting_name = "Accounting";
    let sales_name = "Sales";
    let mut accounting = vec!["Alice", "Bob"];
    let mut sales = vec!["John", "Rick"];
    let mut departments = HashMap::new();
    let possible_commands = ["Add", "List"];
    // possible_commands.insert("Add", add);
    // possible_commands.insert("List", list);

    departments.insert(accounting_name, accounting);
    departments.insert(sales_name, sales);

    greet(&departments, possible_commands);

    loop {
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
                println!("Enter target department and person separated by whitespace");
                println!("Accounting John");

                loop {
                    let mut add_input = String::from("");

                    io::stdin()
                        .read_line(&mut add_input)
                        .expect("Failed to read line");

                    let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

                    println!("{}", add_input.len());

                    if add_input.len() < 2 {
                        println!("Incorrect input, try again");
                        continue;
                    }

                    let target = add_vec[0];
                    let person = add_vec[1];

                    let department = departments.get_mut(target);

                    match department {
                        Some(dep_ref) => {
                            println!("Adding {person} to {target}");
                            let clone = person;
                            dep_ref.push(clone);
                        },
                        None => (),
                    }
                }
            }
            "List" => {
                println!("Listing");
            }
            _ => (),
        }

        // let command_fn = possible_commands.get(&command).copied();
        // match command_fn {
        //     Some(function) => function(&mut departments, accounting_name, "bar"),
        //     None => println!("Incorrect command"),
        // }
    }

    // match departments.get_mut(accounting_name) {
    //     Some(reference) => {
    //         reference.push("Test");
    //     }
    //     _ => (),
    // }

    // match departments.get_mut(accounting_name) {
    //     Some(reference) => {
    //         println!("{:#?}", reference)
    //     }
    //     _ => (),
    // }
}

fn greet(departments: &HashMap<&str, Vec<&str>>, commands: [&str; 2]) {
    println!("-------------");
    println!("There are two departments:");

    for (name, _people) in departments {
        println!("  - {name}");
    }

    println!("We support two commands:");

    for command in commands {
        println!("  - {command}");
    }

    println!("\n");
    println!("Type Quit to quit");
    println!("-------------");
    println!("Please enter your command:");
}

// fn add(departments: &mut HashMap<&str, Vec<&str>>, department_name: &str, person: &str) {
//     println!("Adding");
//     let temp = person;
//     match departments.get_mut(department_name) {
//         Some(reference) => {
//             reference.push(temp);
//         }
//         _ => (),
//     }
// }

// fn list(departments: &mut HashMap<&str, Vec<&str>>, department_name: &str, person: &str) {
//     println!("Listing");
// }
