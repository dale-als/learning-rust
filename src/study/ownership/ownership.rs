fn main() {
    let s01 = String::from("hello");
    let s02 = s01; // to not move and just copy - use s02 = s01.clone()

    // println!("{}, world!", s01); - error, cause s01 was moved to s02

    let s1 = String::from("Test");
    takes_ownership(s1);

    
    // println!("{s1}"); error coause ownership of s1 was passed to function

    let x = 3;
    make_copy(x);

    println!("{x}");

    let mut s2 = gives_ownership();

    println!("{s2}");

    s2 = takes_and_gives(s2);

    println!("{s2}");


}

fn takes_ownership(string: String) {
    println!("I've just stole a string \"{string}\"")
}


fn make_copy(number: i32) {
    println!("Got number {number}")
}

fn gives_ownership() -> String {
    let s1 = String::from("take this");
    s1
}

fn takes_and_gives(str: String) -> String {
    str
}
