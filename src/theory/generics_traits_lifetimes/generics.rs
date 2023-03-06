// When Rust compiles this code, it performs monomorphization.
// During that process, the compiler reads the values that have been used in Option<T>
// instances and identifies two kinds of Option<T>: one is i32 and the other is f64.
// As such, it expands the generic definition of Option<T> into two definitions
// specialized to i32 and f64,
// thereby replacing the generic definition with the specific ones.

//using generically typed sctuct
struct Point<T> {
    x: T,
    y: T,
}

struct PointTwo<T, Z> {
    x: T,
    y: Z,
}

//generically typed enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//generically typed methods will work with any instances of PointTwo
//By declaring T as a generic type after impl, Rust can identify that the
//type in the angle brackets in Point is a generic type rather than a concrete type
impl<T, Z> PointTwo<T, Z> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &Z {
        &self.y
    }

    // Generic type parameters in a struct definition aren’t always the same
    // as those you use in that same struct’s method signatures.
    // The method creates a new PointTwo instance with the x value from the self Point
    // (of type T) and the y value from the passed-in Point (of type Z2).
    // look for example below
    // generic parameters T2 and Z2 are declared after fn mixup,
    // because they’re only relevant to the method.
    fn mixup<T2, Z2>(self, other: PointTwo<T2, Z2>) -> PointTwo <T, Z2> {
        PointTwo {
            x: self.x,
            y: other.y
        }
    }
}

impl PointTwo<f64, f64> {
    // only instances of PointTwo containing f32 at x and y will have that method
    fn sum(&self) -> f64 {
        &self.x + &self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    // println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    // println!("The largest char is {}", result);

    let raw_coords = Point { x: 5, y: 10 };
    let precise_coords = Point { x: 5.5, y: 10.1 };
    // let wont_work = Point { x: 5.5, y: 10 };
    let will_work = PointTwo { x: 5, y: 10.5 };


    let have_f64_impl = PointTwo { x: 3.0, y: 2.0 };

    println!("{}", have_f64_impl.sum());
    // println!("{}", will_work.sum()) -- method .sum() wont work

    let p1 = PointTwo { x: 5, y: 10.4 };
    let p2 = PointTwo { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// genericly typed function, works with any type impl ordering i.e. char and i32
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // fn largest is generic over some type T
    //This function has one parameter named list, which is a slice of values of type T.
    // The largest function will return a reference to a value of the same type T.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
