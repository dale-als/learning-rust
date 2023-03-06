// This is an example and reference for use and mod

mod back_of_house;
mod front_of_house;

use crate::restaraunt::front_of_house::hosting; // bringing path into scope works like "importing" module into current scope
                                                // we can now write just "hosting" instead of full path

// ---- Idiomatic Paths
use crate::restaraunt::front_of_house::hosting::add_to_waitlist; // -- and then just using add_to_waitlist() is discouraged
                                                                 // otherwise it is unclear as to where add_to_waitlist is defined. So it's better to use ...::hosting and use it as hosting::add_to_waitlist()
                                                                 // it's a conventional way to bring paths to functions into scope

use std::collections::HashMap; // for struct and enums it's conventional to bring full path into scope unless they have the same name

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }
// ---

// --- Nested Paths
// use std::fmt;
// use std::io;
// use std::{fmt, io} -- nested path, same as two lines on top

// use std::io;
// use std::io::Write;
// use std::id::{self, Write} -- nested path using self to import io from io, same as two lines on top
// ---

use std::collections::*; // * is a glob operator, used to bring all public items inside std::collections into scope
                         // mostly used when testing as with glob its unclear what came from use and what defined locally

pub use crate::restaraunt::back_of_house::food::Breakfast as SummerBreakfast; // can also use "as" to create Aliases and pub to reexport path to external code

pub fn eat_at_restaraunt() -> String {
    //Absolute path
    crate::restaraunt::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    // let bad_meal = back_of_house::Breakfast {
    //     toast: String::from("Wheat"),
    //     seasonal_fruit: String::from("Oranges"),
    // }; -- cannot create an instance of breakfast like that, cause seasonal_fruit field is private (isn't pub)

    let mut meal = back_of_house::food::Breakfast::breakfast_with_toast("buckwheat");

    meal.toast = String::from("Rye"); // can access and change toast field, cause it's pub

    // meal.seasonal_fruit = "apples"; -- error, cause seasona_fruit field isn't pub

    let order1 = back_of_house::Appetizer::Fish;
    let order2 = back_of_house::Appetizer::Meat;

    hosting::add_to_waitlist();

    let summer_meal = SummerBreakfast::breakfast_with_toast("Rye");

    format!(
        "I'm having {} toast with {} for breakfast!",
        meal.toast,
        meal.seasonal_fruit()
    )
}

mod customer {
    pub fn eat_at_restaraunt() {
        // hosting::add_to_waitlist(); -- would drop an error here, cause use in line 56 brought path to "super" scope
        // !!! child mod has it's own scope
        super::hosting::add_to_waitlist(); // -- works
    }
}

fn main() {
    eat_at_restaraunt();
}

fn deliver_order() {}
