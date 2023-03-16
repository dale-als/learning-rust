use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    // closure (giveaway_fallback, line 25) captures the environment where it's defined
    // in that case it captures &self, an immutable ref to self object
    // so when giveway function is used later and executes the closure - it will have ref to self at hand
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap or else method either returns the value of Option if it's Some or
        // calls the closure from the param to get the value (same type as in Option)
        // closure params go between ||, empty || mean closure with no params
        let giveaway_fallback = || -> ShirtColor { self.most_stocked() };
        // user_preference.unwrap_or_else(|| self.most_stocked()) -- equal to next line
        user_preference.unwrap_or_else(giveaway_fallback)
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let example_closure = |x| x;
    let s = example_closure(String::from("value")); // closure param and return types got infered on the first use
                                                    // let n = example_closure(5); -- error cause type is different to the first use

    // closures can borrow mutably, immutably or take ownership,
    // compiler will decide what to do depending on the code in the closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // fnOnce, fnMut, fn
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // fnOnce, fnMut, fn
    let mut borrows_mutably = || list.push(7);
    // println!("After defining mutable closure: {:?}", list); -- won't work cause list borrowed mutably by closure
    borrows_mutably();
    println!("After calling mutating closure: {:?}", list);

    // fnOnce
    thread::spawn(move || println!("From thread: {:?}", list)); // uses keyword to move ownershi
                                                                // example - pass ownership to another thread

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut counter = 0;
    list.sort_by_key(|r| {
        counter += 1;
        r.width
    });
    println!("Took {} times to sort {:?}", counter, list);
}
