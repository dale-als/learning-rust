use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 12);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Purple"), 111);
    scores.insert(String::from("Green"), 77);

    let team_name = String::from("Blue");

    // .get() gets Option<&V> in case theres no value with provided key
    // .copied() copies Option<V> from Option<&V> to get a value, not a reference
    // .unwrap_or() returns default (param) is Option<V> is None
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        // println!("{key}, {value}");  will print entries in arbitary (random) order
    }


    // for types that can be copied (implement Copy trait), like i32 in "value" here - values are copied
    // for owned types like String, "field_name" here - values are moved and won't be accessible later on
    let field_name = String::from("Magenta");
    let value = 14;

    scores.insert(field_name, value);
    // println!("{field_name}, {value}"); -- error - borrow of moved value: `field_name`
    

    // -- overwriting an existing value
    scores.insert(String::from("Red"), 12);
    scores.insert(String::from("Red"), 15); // "Red" is now 15


    // -- add a value only if a keys isn't present
    // .entry() returns and enum called Entry, representing a value that might or might not exist
    // .or_inster() a method of Entry, returns mut reference to value if key (provided to entry) exists
    // or inserts param as value for key (provided to entry) and returns mut ref to new value
    scores.entry(String::from("Ivory")).or_insert(88); // will insert key Ivory with value 88
    scores.entry(String::from("Blue")).or_insert(88); // won't change hashmap cause key exists

    // println!("{:?}", scores); -- uncomment to check


    // -- updating a values base on the old value
    let mut word_map = HashMap::new();
    let text = "hello world crazy world";

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0); // count is mut reference to value
        // count += 1; -- error cannot use `+=` on type `&mut {integer}`
        *count += 1; // works, * is dereferencing,  following the pointer (the reference) to the actual data
        // so we access data "directly" and update it with +1
        // mutable reference count goes out of scope at the end of the loop


        // fn g1(thing: &Thing) -> &String {
        //     let tmp = *thing;
        //              ┃ ┗━ Point directly to the referenced data.
        //              ┗━━━ Try to copy RHS's value, otherwise move it into `tmp`.

        //     &tmp.field
        // }       
        // https://micouy.github.io/rust-dereferencing/ more on dereferencing

        // By default, HashMap uses a hashing function called SipHash that can provide resistance
        // to Denial of Service (DoS) attacks involving hash tables.
        // This is not the fastest hashing algorithm available
        // It's possible to use different hashers, look it up if HashMap works too slow for you
        
    }

    println!("{:?}", word_map);
    
}