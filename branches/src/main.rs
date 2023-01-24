//if Expressions  --also called arms
/*fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {  //------------------alt block of code to execute should the condition = false. skip if block on next
        println!("condition was false");
    }
}*/

//example without bool
/*fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}*/

//running under certain condition
/*fn main() {
    let number =3;

    if number != 0 {
        println!("number was something other than zero");
    }
} */

//Handling Multiple Conditions with else if expressions
/*fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}*/

//Using if in a let Statement  --assigning the result of an if expression to a variable
/*fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}*/

//example with mismatched types
/*fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}*/

//Repetition with Loops
//Rust has three kinds of loops: 'loop', 'while', and 'for'

//Repeating Code with loop (forever)
/*fn main() {
    loop {
        println!("again!");
    }
}*/

//Returning Values from Loops
/*fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}*/

//Loop Labels to Disambiguate Between Multiple Loops
/*fn main() {
    let mut count = 0;
    'counting_up: loop {    //loop label
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
}*/

//Conditional Loops with while  --eliminates lots of required nesting involving 'loop, if, else, and break' 
/*fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}*/

//Looping Through a Collection with for
/*fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}*/

//concise alternative 
/*fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}*/

//using 'rev' method to reverse the range
/*fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}*/

/*to practice these concepts try building programs which:
convert temperatures between fahrenheit and celsius
generate the nth Fibonacci number
print the lyrics to the Christmas carol "The Twelve Days of Christmas" (taking advantage of the song's repetition)
