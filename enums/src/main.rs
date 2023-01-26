/*
fn main() {
    println!("Hello, world!");
}
*/

//Defining and Enum -- IpAddrKind is now a custom data type that we can use elsewhere in our code.
/*
enum IpAddrKind {
    V4,
    V6,
}
*/

//Enum values  --instances of each of the two variations
/*
fn main() {

}

// instances of the twp variants
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// function defined to take any 'IpAddrKind:'
fn route(ip_kind: IpAddrKind) {}

//variants of the above function
route(IpAddrKind::V4);
route(IpAddrKind::V6);
*/


//Storing the data and 'IpAddrKind' variant of an IP address using a struct
/*enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

//representing the same concept in a more concise way using an enum
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

//storing V4 addresses as four u8 values while expressing V6 addresses 
//as one string value is impossible with a struct, 
//so instead we use enums as shown below:
enum IpAddr {
    V4(u8, u8, u8, u8),
    V8(String),
}

let home - IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
*/

// a 'Message' enum whose variants each store different amounts and types of values
/*
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

This enum has four variants with different data types:
'Quit' has no data associated with it at all.
'Move' has named fields like a struct does.
'Write' includes a single 'String'.
'ChangeColor' includes three 'i32' values.

The following structs could hold the same data that the preceding enum variants hold:

struct QuitMessage; //uint struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

Just as we're able to define methods on structs using 'impl', 
we're also able to define methods on enums. 
Here's a method named call that we could define on our 'Message' enum:

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

body of the message would us 'self' to get the value that we called the method on.
*/

//The Option Enum and Its Advantages Over Null Values
/* an enum that can encode the concept of a value being present or absent 
'Option<T>':

enum Option<T> {
    None,
    Some(T),
}

'Option<T>' enum is so useful that it's even included in the prelude;
you don't need to bring it into scope explicitly. Its variants are also included in the prelude:
you can use 'Some' and 'None' directly without the 'Option::' prefix. The 'Option<T>' enum is 
still just a regular enum, and 'Some(T)' and 'None' are still variants of type 'Option<T>'.

<T> is a generic type parameter, respectively meaning the 'Same' variant of the 'Option' enum can hold one piece 
of data of any type, and that each concrete type that gets used in place of 'T'
makes the overall 'Option<T>' type a different type. Here are some examples of using 'Option' values
to hold number types and string types.

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

In general, in order to use an Option<T> value, 
you want to have code that will handle each variant. 
You want some code that will run only when you have a Some(T) value, 
and this code is allowed to use the inner T. You want some other code to run 
if you have a None value, and that code doesn’t have a T value available. 
The match expression is a control flow construct that does just this when 
used with enums: it will run different code depending on which variant of 
the enum it has, and that code can use the data inside the matching value.
*/

//The match Control Flow Contruct

//An enum and a match expression that has the variants of the enum as its patterns
/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

A 'match' keyword is listed followed by an expression, 
which in this case is the value 'coin'.

This seems very similar to an expressio used with 'if', but there's a big difference:
with 'if', the expression needs to return a Boolean value, but here, 
it can return any type. The type of 'coin' in this example is the 'Coin' 
enum that we defined on the first line.

Next are 'match' arms. An arm has two parts: a pattern and some code. The first
arm here has a pattern that is the value 'Coin:Penny' and then the => operator
that separates the pattern and the code to run. The code in this case is just the
value '1'. Each arm is separated from the next with a comma.

When the 'match' expression executes, it compares the resulting value against
the pattern of each arm, in order.If a pattern matches the value, the code associated 
with that pattern is exevuted. If that pattern doesn't match the value,
execution continues to the next arm, much as in a coin-sorting machine. We can
have as many arms as we need: in the demonstration above, our 'match' has four arms.

The code associated with each arm is an expression, adn the resulting value of 
the expression in the matching arm is the value that gets returned for the entire 'match' expression.

We don't typically use curly brackets if the match arm code is short, 
as it is in the example, where each arm just returns a value.
If you want to run multiple lines of code in a match ar, you must use curly brackets,
and the comma following the arm is then optinoal. For example, the following
code prints "Lucky penny!" every time the method is called with a Coin::Penny, 
but still returns the last value of the block, 1:

fn value_in_cents(coin: coin) -> u8 {
    match coin {
        Coin::Penny =. {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

*/

//Patterns that Bind to Values
/*
fn main() {

}
//A Coin enum in which the Quarter variant also holds a UsSate value
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//match expression to add variable called 'state' to the pattern that matches
//values of the variant 'Coin::Quarter'. When a 'Coin::Quarter' matches, the 'state'
//variable will bind to the value of that quarter's state.
//Then we can use 'state' in the code for that arm, like so:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

*/

//Matching with Option<T>

//function that uses a match expression on an Option<i32>
/*
fn plus_one(x: Options<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

Combining 'match' and enums is useful in many situations. You'll see 
this pattern a lot in Rust code: 'match' against an enum, bind a variable to 
the data inside, and then execute code based on it. It's a bit tricky at first,
but once you get used to it, you'll wish you had it in all alnguages.
It's consistently a user favorite.

//Matches are exhaustive 
//we must exhaust every last possibility in order for the code to be valid.
//this will return an error because we did not consider the above statement.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i +1),
    }
}

*/

//Below is a dice game where if you roll a 3 you get a fancy hat and if
//you roll a 7 it gets taken away

//Catch-all Patterns and the _ Placeholder
/*
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

//new rules
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

//explicit nonuse of any other value that doesn't match a pattern in an 
//earlier arm, and we don't want to run any code in this case.
let dice_roll =9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => 90,
}

fn add_fancy_hat() {}
fn remove_fancy_hat(){}
*/

//Concise Control Flow with if let
/*
The if let syntax lets you combine if and let into a less verbose 
way to handle values that match one pattern while ignoring the rest. 
Consider the program in Listing 6-6 that matches on an Option<u8> value 
in the config_max variable but only wants to execute code if the value 
is the Some variant.

//A 'match' that only cares about executing code when the value is 'Some'
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

If the value is Some, we print out the value in the Some variant by binding 
the value to the variable max in the pattern. We don’t want to do anything 
with the None value. To satisfy the match expression, we have to add _ => ()
after processing just one variant, which is annoying boilerplate code to add.

Instead, we could write this in a shorter way using if let. 
The following code behaves the same as the match in the previous example.

let config_max = Some(3u8);
if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
}

If we wanted to count all non-quarter coins we see while also announcing
the state of the quarters, we could do that with a match expression like this:

let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

or we could use an 'if let' and 'else' expression like this:

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!(State quarter from {:?}!, state);
} else {
    count += 1;
}

If you have a situation in which your program has logic that is too verbose
to express using a 'match', remember that 'if let' is in your Rust toolbox as well.










