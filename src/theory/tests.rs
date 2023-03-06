fn main() {}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be > 0");
        } else if value > 100 {
            panic!("Guess value must be <= 100");
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// private (not pub) function can be tested
// items in child modules can use the items in their ancestor modules
// therefore mod tests can access call my name
fn call_my_name(name: &str) -> String {
    format!("Hello")
}

// conventionally unit tests should live in the file (unit) that they are testin
// in the separate inner module tests with #[cfg(test)]
#[cfg(test)] // mod test will only be compiled while cargo test and ignored while cargo build
mod tests {

    use super::*; // tests is an inner module
                  // making outer module (main.rs) available here

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // calls panic if assert fails
    }

    #[test]
    fn another() {
        // panic!("fail"); each test run in separate thread
        // so panic in the test won't fail whole test suit, just a single test
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    #[ignore] // will be ingored
    fn correct_name() {
        let result = call_my_name("John");
        assert!(
            // custom error message after assertion
            result.contains("John"),
            "Greeting did not contain name, result was {}",
            result
        );
    }

    #[test]
    #[should_panic] // testing that our code panics when it's needed
    fn incorrect_guess() {
        Guess::new(356);
    }

    #[test]
    #[should_panic(expected = "Guess value must be > 0")]
    // expectes specific panic message
    fn guess_too_small() {
        Guess::new(-10);
    }

    #[test]
    #[should_panic(expected = "Guess value must be <= 100")]
    // expectes specific panic message
    fn guess_too_big() {
        Guess::new(666);
    }

    #[test]
    fn it_returns_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore] // will be ingored
    fn ignored() {
        assert_eq!(2, 3);
    }
}

// cargo test -- --test-threads=1 //  limit thread for tests, 1 thread = no parallelism
// cargo test -- --show-output // displays fn outputs like println! in tests output (verbose)
// cargo test guess_too_small // run single test
// cargo test guess // run test containing "guess" in the name
// cargo test -- --ignored // runs only ignored tests
// cargo test -- --include-ignored // runst all tests including ignored
