pub fn nth_fibonacci(n: i32) -> i32 {
    if n < 2 {
        return 0;
    } else if n == 2 {
        return 1;
    }
    
    let mut n1 = 0;
    let mut n2 = 1;
    let mut counter = 0;

    while counter < n - 2 {
        let temp = n1 + n2;

        n1 = n2;
        n2 = temp;
        counter += 1;
    }

    return n2;
}