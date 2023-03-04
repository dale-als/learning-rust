use std::fs::File;

fn main() {
    // panic!("crash and burn"); -- excplicilty calling panic
    let v = vec![1, 2, 3];

    //v[99]; -- implicitly causing panic by accesing out of index bounds

    let greeting_file_result = File::open("hello.txt"); // returns Result


}