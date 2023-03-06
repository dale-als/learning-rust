enum IpAddrVersion {
    V4,
    V6,
}

struct IpAddr {
    version: IpAddrVersion,
    address: String,
}

enum IpAddrEnum { //enum variants can store data as well
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrSpecificEnum { // variant can store different types and amounts of data
    V4(u8, u8, u8, u8),
    V6(String)
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message { //we can define methods for enums as we do for structs
    fn call (&self) {
        println!("{:#?}", self);
    }
}

// enum Option<T> { // option enum to display absense of data, just for example. built-in into rust, can be used just as None and Some
//     None,
//     Some(T),
// }

pub fn main() {
    let four = IpAddrVersion::V4;
    let six = IpAddrVersion::V6;

    test(four);
    test(six);

    let home = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1")
    };

    let home2 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrEnum::V6(String::from("::1"));

    let home3 = IpAddrSpecificEnum::V4(127, 0, 0, 1);
    let loopback3 = IpAddrSpecificEnum::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let mut absent_number: Option<i32> = None;

    // let some_sum = some_number + 3; -- error, cause some_number is not an integer, but an Option<integer>
    // we need to convert Option<T> to T, to use it's

}

fn test(ip_version: IpAddrVersion) { //accepts both IpAddrVersions

}

