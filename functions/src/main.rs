
//Functions
/*fn main() {
    println!("booty!");

    another_function();
}

fn another_function() {
    println!("Cheeks.");
}*/

//Parameters  --special variables defined for the function's signature  
//            --concrete values are called arguments
/*fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}*/

/*fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}*/

//Satements and Expressions  
//--(statements)instructions that perform action & don't return a value
//--(expressions)evaluate to a resultant value
/*fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}*/

//Functions with Return Values
/*fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}*/

//Another Example (expression)
/*fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}*/

//Another Example (statement)
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}