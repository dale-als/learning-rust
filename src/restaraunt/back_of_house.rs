pub mod food;

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); // super basicaly works as .. in filesystem
}

fn cook_order() {}

pub enum Appetizer { // if enum is public, all of it's options are private as well
    Fish,
    Meat
}