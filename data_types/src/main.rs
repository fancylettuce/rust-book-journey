
//Floating-Point Types
/*fn main() {
    let x = 2.0; //f64

    let y: f32 = 3.0; //f32

    let z = x + y; //testing mathematic functionality

    println!("x + y = {z}");
}*/

//Numeric Operations
/*fn main() {
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Results in -1

    //remainder
    let remainder = 43 % 5;
}*/

//The Boolean Type
/*fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}*/

//The Character Type
/*fn main() {
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyes_cat = '😻';
}*/

//-------Compound Types------

//The Tuple Type
/*fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}*/

//Tuple Pattern Matching --destructure tuple value
/*fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}*/

//Direct Tuple Element Access --(.) followed by index of the value we want to access
/*fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1

    let one = x.2;
}*/

//The Array Type
/*fn main() {
    let a = [1, 2, 3, 4, 5];
}*/

//arrays are best used when you know the number of elements doesn't need to change
/*let months = ["January", "February", "March", "April", "May", "June", "July",
 "August", "September", "October", "November", "December"];*/

//Writing An Array's Type
//let a: [i32; 5] = [1, 2, 3, 4, 5];
//--  a(rray): [element_type; element_qty] = [t, h, e, a, r, r, a, y];
//--  for initiating consistent values for each element in a concise way, let a = [initial_value; array_length];

//Accessing Array Elements
/*fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}*/

//Invalid Array Element Access --example of rust's memory safety principles. e.g. blocks access to invalid memory
/*use std::io

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}*/

