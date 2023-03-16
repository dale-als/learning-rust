fn main() {
    let v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];

    let mut v1_iter = v1.iter();

    v1_iter.next(); // consumes next element from vector and returns it

    println!("After calling next");

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // vals from iterator created by iter method are immutable
    // use iter_mut to get mutable
    for x in v2.iter_mut() {
        *x += 1;
    }

    for x in v2.iter() {
        println!("{x}");
    }

    // ITERATOR CONSUMERS (like .sum()) return result, but consume the iterator
    // mostly cause they use .next() internaly

    let test_iter = v1.iter();
    let sum: i32 = test_iter.sum(); // test iter gets consumed here

    println!("The sum of v1 is: {sum}");

    // for x in test_iter { -- error cause test_iter is consumed
    // println!("{x}");
    // }

    // ITERATOR ADAPTORS return new array but doesn't do anything unless used, read warning
    v2.iter().map(|x| x + 1); // closure won't be called here

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect(); // now iterator adaptor result is collected and used

    for x in v3.iter() {
        println!("{x}");
    }

    // example of an array filter, into_iter() used to take owership of a iter as it's getting changer (6 being dropped)
    let v4: Vec<i32> = v3.into_iter().filter(|x| x > &6).collect();

    for x in v4 {
        println!("{x}");
    }
}
