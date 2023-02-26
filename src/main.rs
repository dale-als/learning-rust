mod garden;
mod restaraunt;

use crate::garden::vegetables::Asparagus;



fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    println!("{}", restaraunt::eat_at_restaraunt());
}