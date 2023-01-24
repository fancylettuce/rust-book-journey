
/*Ownership Rules 
--Each value in rust has an owner
--There can only be one owner at a time --highlander
--When the owner goes out of scope, the value will be dropped*/

/*fn main() {
    let s = "hello"; // s refers to a string literal -- value is hardcoded into program text
}*/


//A Variable and the scope in which it is valid
/*fn main() {    //s is invalid here as it's not yet declared
    let s = "hello"; //is valued from this point forward

    //do stuff with s
        //scope is now over and s is no longer valid
}*/

//The String Type

//Creating a string from a string literal using the 'from' function
/*fn main() {
    let s =string::from("hello");
}*/

//mutable string types
/*fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // this will print 'hello, world!'
}*/

//Memory and Allocation
/* In memory models without garbage collection, we need to manually identify when memoru is no longer being used and pair exactly one 'allocate' with exactly one 'free'.
In laguages with a Garbage Collector, the GC keeps track of and cleans up unused memory automatically.
Rust does things differently: memory is automatically returned once the variable that owns it goes out of scope.*/

//Scope example of string
/*fn main() {
    let mut s = String::from("hello");  // s is valid from this point forward

    s.push_str(", world!");

    println!("{}", s);

    // do stuff with s
        // this scope is now over, and s is no longer valid
}*/

// when a variable goes out of scope, rust calls the 'drop' function, which is called automatically at the closing curly bracket
/* this is similar to Resource Acquisition Is Initialization (RAII) patterns 
utilized to deallocate resources at the end of an item's lifetime in languags such as C++*/

//Variables and Data Interacting with Move

//Assigning the integer value of variable x to y
/*fn main() {
    let x = 5;
    let y = x;
}*/

//Assigning the string value
/*fn main() {
    let s1 = String::from("hello");
    let s2 = s1
    
    println!("{}, world!");
}*/

/* A String is made up of three things:
a pointer to the memory that holds its contents
a length (how much memory, in bytes, the contents of the 'String' are currently using)
a capacity (total amount of memory, in bytes, that the 'String' has received from the allocator)
this group of data is stored on the stack.*/

// freeing memory twice can result in what's known as a 'double free error', a memory safety bug which can result in security vulnerabilities.

/* in Rust, the concept of copying the pointer, length, and capacity 
without copying the data is known as a move. (as rust invalidates the first variable)
This is similar to terms named 'shallow copy' and 'deep copy' in other languages.
Rust will never automatically create 'deep' copies of youur data. Therefore,
any automatic copying can be assumed to be inexpensive in terms of runtime performance.*/

//Variables and Data Interacting with Clone
/* if we do want to deeply copy the heap data of the 'String', and not just
stack, we can use a common method called 'clone'*/

//Example of clone
/*fn main() {

    let s1 = String::from("yo");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}*/

//Stack-Only Data: Copy
/*fn main() {

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}*/

/* as a general rule, any group of simple scalar values can implement Copy, 
and nothing that requires allocation or is some form of resource can implement Copy. 

Here are some of the types that implement Copy:
All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating-point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. 
For example, (i32, i32) implements Copy, but (i32, String) does not.*/

//Ownership and Functions
/*fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);    //s's value moves into the function...
                          //...and is no longer valid here

    let x = 5;  // x comes into scope

    makes_copy(x);  // x would move into the function, 
                    // but i32 is Copy, so it's okay to use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved,
    // nothing special happens.

    fn takes_ownership(some_string: String) {  // some_string comes into scope
        println!("{}", some_string);
    }  // Here, some_string goes out of scope and 'drop' is called. The
      // backing memory is freed.

    fn makes_copy(some_integer: i32) {  // some_integer comes into scope
        println!("{}", some_integer);
    }  // Here, some_integer goes out of scope. Nothing special happens.
} */

//Return Values and Scope

//Example of a function that returns some value, similarly annotated to the last example
/*fn main() {
    let s1 = gives_ownership();
    // gives_ownership moves its return value into s1

    let s2 = String::from("hello");  //s2 comes into scope

    let s3 = takes_and_gives_back(s2);  
    //s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3.

} //Here, s3 goes out of scope and is dropped. s2 was moved, 
// so nothing happens. s1 goes out of scope and is dropped

fn gives_ownership() -> String {
// gives_ownership will move its return value into the function that calls it


    let some_string = String::from("yours");  // some_string comes into scope

    some_string
    // some_string is returned and moves out to the calling function

}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a String comes into scope


    a_string // a_string is returned and moves out to the calling function
}*/

/* The ownership of a variable follows the same pattern every time: 
assigning a value to another variable moves it. When a variable that includes 
data on the heap goes out of scope, the value will be cleaned up by drop 
unless ownership of the data has been moved to another variable.*/

//Returning ownership of parameters
/*fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() returns the length of a string

    (s, length)
}*/

//References and Borrowing
/*a reference is guaranteed to point to a valid value 
of a particular type for the life of that reference; unlike a pointer.*/

/*definition and usage of a caluculate_length function with a reference
to an object as a parameter instead of taking ownership of the value.*/

//string length calculator function
/*fn main() {
    let s1 = String::from("roll tide");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}*/

/* the '&' represent references which allow you to refer to some value
 without taking ownership of it.*/

/* the opposite of referencing by using '&' is dereferencing, which accomplished 
with the dereference operator '*'.*/

//example of a function call
/*fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}*/ // Here s goes out of scope. Because it does not have ownership of what 
  // it refers to, it is not dropped.

/* when functions have references as parameters instead of actual values, 
we wont need to return the values in order to give back ownership, 
because we never had ownership*/

/* the action of creating a reference is called 'borrowing'. As in real life
if a person  owns something, you can borrow it from them. When you're done, 
you have to give it back. You don't own it.*/

//Attempting to modify a borrowed value --throws an error
/*fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}*/

/* this restriction, which prevents multiple mutable references to the same data 
at the same time, allows for mutation in a controlled fashion. This can 
help mitigate data races at compile time. A data race is similar to a 
race condition and happens when these three behavior occur:
 Two or more pointers access the same data at the same tim.
 At least one of the pointers is being used to write the data
 There's no mechanism being used to synchronize access to the data*/

//using a mutable reference to modify a borrowed value
/*fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}*/

 // This causes undefined behavior and can be difficult to diagnose and
 // fix when you're trying to track them down at runtime; Rust prevents
 // this problem by refusing to compile code with data races.

 // we can use curly brackets to create a new scope, allowing for multiple
 // references, just not simultaneous ones --compiles with warnings*
 /*fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;

    {
        let r1 = &mut s;
    }  //r1 goes out of scope here, so we can make a new reference with no problem

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; //no problem
    let r2 = &s; //no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this post

    let r3 = &mut s; //no problem

    println!("{}", r3);
 }*/

//we cannot have a mutable reference while we have an immutable one to the
// same value. Multiple immutable references are allowed because no one who
// is reading the data has the ability to affect anyone else's reading of data.

/*in rust, the compiler guarantees that references will never be 'dangling references':
in the event you reference data, the compiler will ensure that the data
will not go out of scope before the reference to the data does.*/

// attempting to create a dangling reference  --error refers to 'lifetimes'
/*this function's return type contains a borrowed value, 
but there is no value for it to be borrowed from*/

/*fn main() {
    let dangle = dangle();
    let no_dangle = no_dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

// a loser look
/*fn dangle() -> &String { //dangle returns a reference to a string

    let s = String::from("hello"); // s is a new string

    &s // we return a reference to the String, s
}*/ // Here, s goes out of scope, and is dropped. It's memory goes away.
  // Danger!

// the solution is to return the String directly
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}*/

//The Style Type

// first_word function that returns a byte index value into the string parameter
/*fn main() {
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}*/

//storing the result from calling the 'first_word' function then changing the 'String' contents
/*fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get the value s

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now invalid.
}*/

//String Slices
/* a string slice is a reference to a part of a string.
below is an example of how a string slice looks:

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

We create slices using a range within brackets by specifying 
[starting_index..ending_index], where starting_index is the first position 
in the slice and ending_index is one more than the last position in the slice. 
Internally, the slice data structure stores the starting position and the length 
of the slice, which corresponds to ending_index minus starting_index.

with Rust's '..' range syntax, if you want to start at index 0, you can drop 
the value before the two periods. In other words, these are equal:

let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

if your slice includes the last byte of the String you can drop the trailing 
number. That means these are equal:

let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

You can also drop both values to take a lice of the entire string. 
so these are equal:

let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

*/

// Rewriting 'first_word' to return a slice. 
// signifies "string slice" is written as '&str':

/*fn main() {
    let s = String::from("hello");

    let slice = &s[0..s];
    let slice = &s[..s];
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}*/

// will throw a compile-time error
/*fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}*/

/*String Literals as Slices

let s = "Hello, world!";

The type of s here is &str: 
it’s a slice pointing to that specific point of the binary. 
This is also why string literals are immutable; 
&str is an immutable reference.*/

/*String Slices as Parameters

fn first_word(s: &String) -> &str{ //okay signature

}

fn first_word(s: &str) -> &str { 
    //more efficient signature because it allows us to use
    //the same function on both '&String' values and '&str' values.
    //this is an improvement on the 'first_word' function by using
    //a string slice for the type of the 's' parameter.

}
*/

/* If we have a string slice, we can pass that directly. 
If we have a String, we can pass a slice of the String 
or a reference to the String.
This flexibility takes advantage of deref coercions

Defining a function to take a string slice instead 
of a reference to a String makes our API more general and useful 
without losing any functionality: 

fn main() {
    let my_string = String::from("hello world");
    
    // 'first_world' works on slices of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 'first_word' works on slices of 'String's, which are equivalent
    // to whole slices of 'string's
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first'_word' works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // this works without the clice syntax 
   // because string literals *are* atring slices already:
   let word = first_word(my_string_literal);
}
*/

/* Other Slices
aside from string slices, which are specific to strings, there are more
general types of slices too. Consider this array:

let a = [1, 2, 3, 4, 5];

just as we have referred to a part of a string, we might also want to 
refer to part of an array. That would look like this:

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &2, 3]);

This slice has the type '&[i32]. It works the same way as string slices do,
by storing a reference to the first element and a length.
*/










