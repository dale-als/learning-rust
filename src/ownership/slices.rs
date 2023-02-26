fn main() {
    let s = String::from("hello world");
    let len = s.len();

    let hello = &s[0..5]; // slice can be represented by two numbers:
    // starting index (0) and length (end index 5 - starting 0)
    // slices are IMMUTABLE REFERENCES

    let hello = &s[..5]; // same as previous line
    let world = &s[6..len];
    let world = &s[6..]; // same as previous line

    let hello_world = &s[0..len];
    let hello_world = &s[..]; // same as previous line

    let hello = first_word(&s);

    // s.clear() - error - s is taken as immutable reference in slices,
    // so can't be taken as mutable for clearing
    let string_slice = "Hello, world!"; //type is &str - slice pointing to memory
    let slice_of_a_slice = &string_slice[..5];

    println!("{slice_of_a_slice}");

    let array = [1, 2, 3, 4, 5, 6];
    let array_slice = &array[1..3];
    let slice_literal = &[2, 3];

    assert_eq!(array_slice, slice_literal);
    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}