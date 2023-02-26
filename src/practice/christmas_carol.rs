pub fn sing() {
    let mut line_number = 0;
    let days = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh",
      "Eighth", "Nineth", "Tenth", "Eleventh", "Twelfth"];
    
    let gifts = ["A Partridge in a Pear Tree", "Two Turtledoves", "Three French Hens", "Four Calling Birds",
      "Five Gold Rings", "Six Geese a-Laying", "Seven Swans a-Swimming", "Eight Maids-a-Milking", "Nine Ladies Dancing",
      "Ten Lords a-leaping", "Eleven Pipers Piping", "Twelve Drummers Drumming"];

    while line_number < 12 {
        let line = format!("On the {} Day of Christmas, My True Love Gave to Me: \n{}.", days[line_number], gifts[line_number]);

        println!("{line}");

        line_number += 1;
    }
}