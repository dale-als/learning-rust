use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    

    loop {
        println!("Enter the number from 1 to 10");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Not a number, try again!");
                continue;
            },
        };
    

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win!");
                break;
            },
        }
    }

    println!("Test {secret_number}");
}
