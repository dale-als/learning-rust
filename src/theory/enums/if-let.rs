fn main() {
    let config_max = Some(3u8);
    let config_empty: Option<u8> = None;

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //same as previous match, if (and else ifs) would work as arms here, else is catch-all
    if let Some(value) = config_max {
        println!("The maximum is configured to be {}", value);
    } else {
        println!("Value is not set");
    }

    if let Some(value) = config_empty {
        println!("The maximum is configured to be {}", value);
    } else {
        println!("Value is not set");
    }
}