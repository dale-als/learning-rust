mod restaraunt;

// fn main() {
//     println!("{}", restaraunt::eat_at_restaraunt());
// }


fn main() {
    let vector: Vec<i32> = Vec::new(); // explicit type annotain, cause creatin empty and can't infer
    let mut vector2 = vec![1, 2, 3]; // using macro and implicit type, cause have values and can infer

    // vector1.push(4); -- error cause vector should be declared as mutable to be changed
    vector2.push(33);
    
    let third: &i32 = &vector2[2]; // get a reference to a collection element at index

    let third_by_get: Option<&i32> = vector2.get(2); // get an Option for use with match (in case of no element at index)

    match third_by_get {
        Some(element) => println!("Third element is {element}"),
        None => println!("There is no third element")        
    }

    // if you need to crash on out of bound
    // let does_not_exist = &vector2[10]; crash, panic 'index out of bounds'

    // if you need to continue on out of bound
    let does_not_exist_either = vector2.get(100); // no panic, just Option::None


    let mut vector3 = vec![1, 2, 3, 4, 5];


    // --- !! REFERENCING WITH VECTORS
    let first = &vector3[0]; // immutable borrow

    // vector3.push(6);  mutable borrow and error

    // println!("First element {first}");

    // Reason - because vectors put the values next to each other in memory, adding a new
    // element onto the end of the vector might require allocating new memory and 
    // copying the old elements to the new space f there isn’t enough room to put all the 
    // elements next to each other where the vector is currently stored, making first reference

    // --- !!

    for i in &vector3 { // using immutable &vector3 reference to read elements
        println!("{i}");
    }

    for i in &mut vector3 { // using mutable &mut vector3 reference to modify elements
        *i += 10; // * is dereferencing, will learn more later
    }


    // --- !! Using Enum to store multiple data types in the vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int {value}"),
            SpreadsheetCell::Float(value) => println!("Float {value}"),
            SpreadsheetCell::Text(value) => println!("Text {value}"),
        }
    }
}