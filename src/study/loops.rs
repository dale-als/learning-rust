pub fn test_double_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn test_while() -> i32 {
    let mut countdown = 3;

    while countdown > 0 {
        println!("{countdown}!");
        countdown -= 1;
    };

    println!("Liftoff 🔥🔥🔥");
    
    return 4
}

pub fn test_looping(numbers: [i32; 3]) {
    let mut index = 0;
    let length = numbers.len();

    while index < length {
        println!("Current value {}", numbers[index]);
        index += 1;
    }

    for number in numbers {
        println!("Current value {number}");
    }

    for number in (1..6).rev() {
        println!("{number}")
    }
}