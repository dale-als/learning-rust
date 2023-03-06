fn main() {
    //immutable reference
    let s1 = String::from("Immutable reference");
    println!("{}", get_length(&s1));
    // change(&s1); would be an error

    //mutable reference
    let mut s2 = String::from("Mutable reference");

    change(&mut s2);
    
    println!("{s2}");

    // multiple immutable references are ok - no data race;
    // let r1 = &s2;
    // let r3 = &s2;
    
    //multiple mutable references not ok - possible data race if concurrent mutation
    let r2 = &mut s2;
    // let r4 = &mut s2; -- error here

    //both mutable and immutable references at the same time not possible
    // let r5 = &s2; -- error here


    
    // references scope ends with last usage
    let r6 = &s2; // no problem
    let r7 = &s2; // no problem
    println!("{} and {}", r6, r7);
    // variables r6 and r7 will not be used after this point

    let r8 = &mut s2; // no problem
    println!("{}", r8);

    // let ref_to_nothing = dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", now mutated");
}

fn get_length(string: &String) -> usize {
    string.len()
}

// error with dangling references - won't work
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // because this is a ref to s, which will be dropped after dangle function ends
// }
