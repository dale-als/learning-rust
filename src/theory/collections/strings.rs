fn main() {
    // String is built on top of Vector with extra check and restrictions, so can be created likewise
    let s = String::new();

    let literal = "string literal";

    let s = literal.to_string(); // string from string literal, works with every type with Display trait

    let s = "string literal".to_string(); // works like that as well

    let s = String::from("string literal"); // same as before

    let mut foo = String::from("foo");

    foo.push_str("bar"); // remember that a string literal is a slice

    // foo.push_str(s);  --error, push_str takes string slice to not change ownership and s is owned String

    foo.push_str(&s); // -- works fine
    // println!("{s}");  -- s still valid

    let mut lol = String::from("lo");
    lol.push('l'); // takes a single char as param, lol now contains lol

    let hello = String::from("Hello, ");
    let world = String::from("world!");

    let hello_world = hello + &world; // hello_world - "Hello, world!", world - "world!" and hello is moved
    // first argument in + string coercion always gets moved as it has to be of
    // a mutable type and all the rest needs to be slice

    //if don't want to loose ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let tic_tac_toe = format!("{s1}-{s2}-{s3}"); // uses only references

    // println!("{s1}"); still valid

    let hello = String::from("hello");


    // let h = s1[0]; -- error `String` cannot be indexed by `{integer}`
    // !! Why doen't string support integer indexing? Cause they are Vector full of byte data! 
    // each vector element contains not a symbol, but a corresponding byte value
    let helo = String::from("Hola"); // len looks line 4 (and realy is 4 bytes)

    let hello = String::from("Здравствуйте"); // len looks like 12, actually is 24 bytes
    // let answer = &hello[0]; -- anwer is not З but the first byte of З (208), each cyrillic letter takes two

    // each string can be represented as vector of bytes, or vector of chars or vector of grapheme clusters (need external libs)

    let slice = &hello[0..4]; // we cant get a single index, but we can get a slice - slice here = "Зд"
    // let slice = &hello[0..3]; -- panicked at 'byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`'

    // println!("{slice}");

    
    // iterate over chars in string
    for char in hello.chars() { // .chars() takes reference
        // println!("{char}");
    }

    for byte in hello.bytes() { // .bytes() takes reference
        println!("{byte}");
    }

 


}