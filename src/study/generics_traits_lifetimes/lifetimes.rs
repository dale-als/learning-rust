use std::fmt::Display;

// Lifetime Annotations in Struct Definitions

struct ImportantExcerpt<'a> {
    part: &'a str, // &str string slice is a reference
    // This annotation means an instance of ImportantExcerpt can’t outlive
    // the reference it holds in its part field
}

impl <'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
        // elision rules 1 and 3 get's applied here
        // After first rule: self and announcement gets 'a and 'b lifetimes
        // After third rule: return gets a lifetime of self
    }

    

}

fn main() {
    let mut r = 1;
    let s: &'static str = "I have a static lifetime.";
    // static lifetime means that reference live for the entire duration of a program
    // all string literals have static lifetime even without annotation

    // Lifetimes on function or method parameters are called input lifetimes,
    // and lifetimes on return values are called output lifetimes.

    {
        let x = 5;
        // r = &x; --  !!!! error `x` does not live long enough
        // r now contains reference to x
        // x  is not valid outside that scope
    }

    // x is not valid here
    // r is still valid here
    // x "doesn't live long enough" error

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    {
        let string3 = "aardvark";
        let result2 = longest(&string1.as_str(), string3);
        println!("The longest string is {}", result2);
        // ok cause string3 and result2 used in the same scope
    }

    // let mut result3 = "test";

    {
        let string4 = String::from("long string is long");
        // result3 = longest(string1.as_str(), string4.as_str());
        // string4 doesn't live long enough
        // ok cause string4 and result3 used in different scope
        // lifetime of result3 > lifetime of string4
    }

    // println!("The longest string is {}", result3);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Generic Lifetimes in Functions

// fn longest(x: &str, y: &str) -> &str {
//     // won't compile:
//     // missing lifetime specifier
//     // this function's return type contains a borrowed value,
//     // but the signature does not say whether it is borrowed from `x` or `y`
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime Annotation Syntax

// &i32         a reference
// &'a i32      a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime

// Lifetime Annotations in Function Signatures

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // this function declaration means that all the references in the signature
    // Must have the same lifetime 'a
    // compiler won't check how long this lifetime is
    // just that it's the same for all references
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// fn test_longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     // result.as_str() -- error cause we trying to return the reference to a result
//     // which is only valid inside test_longest function
//     // lifetime annotation can't help that
// }

// Lifetime Elision

fn first_word(s: &str) -> &str { // function from /study/ownership/slices.rs
    // owrks without lifetime annotaitions while using references
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// actually works like that, in previous function
// lifetime annotaitions are infered due to lifetime elision rules
// look up lifetime elision part in the Chapter 10.3 of the Rust Book
fn first_word_actual<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// The first rule is that the compiler assigns a lifetime parameter to each
// parameter that’s a reference. In other words, a function with one parameter
// gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two
// parameters gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is that, if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because
// fewer symbols are necessary.


// Generic Type Parameters, Trait Bounds, and Lifetimes Together

// Because lifetimes are a type of generic,
// the declarations of the lifetime parameter 'a and the generic type parameter T
// go in the same list inside the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
