// mod practice;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct UserReference {
//     active: bool,
//     username: &str, -- lifetime specifier error, cause in that case struct is not owning it's data, just a reference
//     email: &str, -- lifetime specifier error, cause in that case struct is not owning it's data, just a reference
//     sign_in_count: u64,
// }

struct Color(i32, i32, i32); // can use tuple structs if field names are redundant
struct AlwaysEqual; // Unit-like struct, if we need type with a special behaviour (trait) and no data

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // area method implementation
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f32) {
        self.width = (self.width as f32 * factor) as u32;
        self.height = (self.height as f32 * factor) as u32;
    }

    fn width(&self) -> bool { //methods can have same names as field
        self.width > 0
    }    
}

// structs can have multiple implementation blocks (not sure why tho, yet)
impl Rectangle {
    // function, associated (implemented for) with a struct, that doesn't need reference istance of self (&self, &mut self or self)
    // often used as a constructors
    fn create_square(size: u32) -> Self { // Self here is not a reference, just an alias for a type, function is associated with
        Self { 
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user1 = User {
        active: false,
        username: String::from("test"),
        email: String::from("test@test.com"),
        sign_in_count: 23
    };

    // println!("{}", user1.email);
    user1.email = String::from("test2@test.com");
    // println!("{}", user1.email);

    let user2 = build_user(String::from("test33@test.com"), String::from("test3"));
    // println!("{}", user2.username);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("entirely@new.com"),
        sign_in_count: user1.sign_in_count
    };

    //struct update syntax - moves data!! from user2 to user4

    let user4 = User {
        email: String::from("test44@new.com"),
        ..user2
    };    

    //println!("{}", user2.username)  -- error here, cause user2.username  was moved to user4.username (see ownership). email wasn't moved
    // and sign_in_count and active were just copied - they are primitives

    // println!("{}", user1.username) -- same error here

    let black = Color(0, 0, 0);
    let assertion = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // println!("Area is {}", area(&rect1));
    // println!("Rectangle is {}", rect1); -- error, cause Rectangle struct
    // doesn't implement Display trait (macro doesn't know what to print)

    // println!("Figure is {:?}", rect1); // -- trying to print using Debug (enabled by #[derive(Debug)]) on the structure
    // println!("Figure is {:#?}", rect1); // -- pretty print Debug

    //NB! println! take a reference, so data is not moved and print to stdout - ie rect1 is available after printind
    // dbg!(rect1);
    // dbg! take ownership of expression (so data is moved) and print to stderror
    // println!("Figure is {:#?}", rect1) -- error, data was moved to dbg!

    
    let rect2 = dbg!(rect1); // dbg! returns ownership of an expression, so rect1 now moved to rect2

    // dbg!(&rect2); // using reference to rect2, so data not moved

    let scale = 3;
    let mut rect3 = Rectangle {
        width: dbg!(10 * scale), // dbg can be used like this
        height: 50
    };

    println!("Area from area method is {}", rect3.area());
    rect3.scale(0.5);
    println!("Area after scaling {}", rect3.area());

    if rect3.width() {
        println!("Rect have a positive width of {}", rect3.width)
    }

    let square = Rectangle::create_square(10);
    println!("{:#?}", square)

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(figure: &Rectangle) -> u32 {
    figure.width * figure.height
}
