//Generic Types, Traits, and Lifetimes

/*
every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete
types or other properties. We can express the behavior of generics or how they 
relate to other generics without knowing what will be in their place when
compiling and running the code.

Functions can take parameters of some generic type, instead of a concrete type 
like 'i32' or 'String', in the same way a function takes parameters with
unknown values to run the same code on multiple concrete values. In fact, 
we've already used generics: 'Option<T>', 'Vec<T>', 'HashMap<K, V>',
'Result<T, E>.

First, we'll review how to extract a function to reduce code duplication.
We'll then use the same technique to make a generic function from two
functions that differ only in the types of their parameters. 

Traits can be used to define behavior in a generic way. You can combine trais 
with generic types to contrain a generic type to accept only those types that
have a particular behavior, as opposed to just any type.

Finally, we'll discuss lifetimes: a variety of generics that give the compiler
information about how references relate to each other. Lifetimes allow us
to give the compiler enough information about borrowed values so that it
can ensure references will be valid in more situations than it could without
our help.
*/

//Removing Duplication by Extracting a Function
/*
Generics allow us to replace specific types with a placeholder that represents 
multiple types to remove code duplication. Before diving into generics syntax,
then, let's first look at how to remove duplication in a way that doesn't involve
generic types by extracting a function that replaces specific values with a
placeholder that represents multiple values. Then we'll apply the same
technique to extract a generic function! By looking at how to recognize duplicated
code you can extract into a function, you'll start to recognize duplicated
code that can use generics.
*/

//We begin eith the short program below, which finds the largest number in a list
/*
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
*/
/*
We store a list of integers in the variable 'number_list' and place a reference
to the first number in the list in a variable named 'largest'. We then iterate
through al the number in the list, and if the current number is greater
thatn the number stored in 'largest', replace the reference in that variable.
However, if the current number is less than or equal to the largest number seen 
so farm the variable doesn't change, and the code moves on to the next number
in the list. After considering all the numbers in the list, 'largest' should
refer to the largest number, which in this case is 100.

We've now been tasked with finding the largest number in two defferent lists 
of numbers. To do so we can choose to duplicate the code from the above example
and use the same logic at two different places in the program; shown below:
*/

//Code to find the largest number in two lists of numbers
/*
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("it's over {}", largest);
    
    let number_list = vec![1102, 34, 9000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("what {}", largest);
}
*/
/*
Although this code works, duplicating code is tedious and error prone. We 
also have to remember to update the code in multiple places when we want
to change it. To eliminate this duplication, we'll create an abstraction by 
defining a function that operates on any list of integers passed in a 
parameter. This solution makes our code clearer and lets us express the 
concept of finding the largest number in a list abstractly.

Below, we extract the code that finds the largest number into a function 
named 'largest'. Then we call the function to find the largest in the two 
lists from the example above. We could also use the function on any other
list of 'i32' values we might have in the future.
*/

//Abstracted code to find the largest number in two lists
/*
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 9000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("what {}", result);
}
*/

/*
The 'largest' function has a parameter called 'list', which represents any 
concrete slice of 'i32' values we might pass into the function. As a result, 
when we call the function, the code runs on the specific values that we pass in.

In summary, here are the steps we took to change the code from above.:

Identify duplicate code.

Extract the duplicate code into the body of the function and specify
the inputs and return values of that code in the function signature

update the two instances of duplicated code to call the function instead

Next, we''ll use generics to reduce code duplication. In the same way that 
the function body can operate on an abstract 'list' instead of specific
values, generics allow code to operate on abstract types.

For ecample, say we had two function; one that finds the largest item in a slice
of 'i32' values and one that finds the largest item in a slice of 'char' values.
How would we eliminate that duplication?
*/

//Generic Data Types
/*
We use generics to create definitions for items like function signatures 
or structs, which we can then use with many different concrete sata types.
Let's first look at how to define functions, structs, enums, and methods
using generics. Then we'll see how generics affect performance.
*/

//In Function Definitions 
/*
When defining a function that uses generics, we place the generics in the
signature of the function where we would ususally specify the data types of
the parameters and return value. Doing so makes out code more flexible and 
provides more functionality to callers of our function while prevening code
duplication.

Continuing with our 'largest' function, the example below shows two functions
that both find the largest value in a slice. We'll then combine these into
a single fucntion that uses generics.
*/

//Two functions that differ only in their names and the types in their sigs
/*
fn largest_i32(list: &[i32]) -> & i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
*/
/*
The 'largest_i32' function is the one we extracted in the example above 
that finds the largest 'i32' in a slice. The 'largest_char' function finds the
largest 'char' in a slice. The function bodies have the same code, so let's
eliminate the duplication by introductin generic type parameter in a single
function.

To parameterize the types in a new single function,we need to name the type 
parameter, just as we do for the value parameters to a function. You can use
any identifier as a type parameter name. But, we'll use 'T' cebause, by convention
type parameter names in Rust are short, often just a letter, and Rust's type-
naming convention is CamelCas. Short for "type," 'T' is the default choice of 
most rust programmers.

Wehn we use a parameter int he body of the function, we have to declare
the parameter name in the signature so the compiler knows what that name
means. Similarly, when we use a type parameter name in a function signature, 
we have to declare the type parameter name before we use it. to define the
generic 'largest' function, place type ame declarations inside angle brackets,
<>, between the name of the function and the parameter list, like this:
*/

// fn largest<T>(list: &[T]) -> &T {

/*
We read this definition as: the function 'largest' is generic over some type 'T'.
This function has one parameter named 'list', which is the slice of values
of type 'T'. The 'largest' function will return a reference to a value of the 
same type 'T'.

The code below shoes the combined 'largest' function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of 'i32' values or 'char' values. 
*/

//The largest function using generic type parameters --won't compile
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest =&list[0];

    for item in list {
        if item ? largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number i {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/
/*
The help text mentions 'std::cmp::PartialOrd, which is a trait. For now, know that
the above error states that the body of 'largest' won't work for all possible
types that 'T' could be. Because we want to compare calues of type 'T' in the body,
we can only use types whose values can be ordered. To enable comparisons, the 
sts has the' std::cmp::PartialOrd', trait that you can implement on types. By
following the help text's suggestion, we restrict the types valid for 'T' to
only those that implement 'PartialOrd' anf this example will compile, because 
the standard library implements 'PartialOrd' on botj 'i32' and 'char'.

//In Struct Definitions

We can also define structs to use a generic type parameter in one or more
fiels using the <> syntax. The code below defines a 'Point<T.' struct to hold
'x' and y'y coordinate values of any type.
*/

//A Point<T> struct that holds x and y values of type T
/*
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0 };
}
*/
/*
The syntax for using generics in struct definitions is similar to that used
in function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we use the generic type in
the struct definition where we would otherwise specify concrete data types.

Note that becuade we've used only one generic type to define 'Point<T>', this
definition says that the 'Point<T>' struct is generic over some type 'T', 
and the fields x and y are both the same type, whatever type they are.
Below is an instance of a 'Point<T>' that has values of different types:

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}

Above, we let the compiler know that the generic type 'T' will be an integer
for this instance of 'Point<T>'. Then when we specify 4.0 for y, which we've
defined to have the same type as x, we'll get a mismatch error.

To define a 'Point' struct where x and y are both generics but could have
different types, we can use multiple generic type parameters. For example, 
The code below shows that we change the definition of 'Point' to be generic
over types 'T' and 'U' where 'x' and 'T' and 'y' is of type 'U'.
*/

//A point<T, U> generic over two types so that x and y can be values of diff types
/*
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let moth_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
*/

//In Enum Definitions
/*
As we did with structs, we can define enums to hold generic data types in their
variants. Let's take another look at the 'Option<T>' enum that the std library provides.

enum Option<T> {
    Some(T),
    None,
}

This definitino should now make more sense to you. As you can see, the 'Option<T>'
enum is generic over type T and has two variants: Some, which hold =s the value
of type 'T', and a 'None' variant that doesn't hold any value. By using the
'Option<T>' enum, we can express the abstract concept of an optional value, 
and becaue 'Option<T>' is generic, we can use this abstraction no matter what
the type of the optional value is..

Enums can use multiple generic types as well. The definition of the 'Result' 
enum that we used earlier is an example.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

The 'Result' enum is generic over two types, 'T' and 'E', and has two variants:
'Ok', which holds a value of type 'T', and 'Err', which holds a value of type 'E'.
This definition makes it conenient to use the 'Result' enum anywhere we have
an operation that might succeed(return a value of some type 'T') or fail(return 
an error of some type 'E'). In fact, this is what we used to open a file where
'T' was filled in with the type 'std::fs::File' when the file was openened 
successfully and 'E' was filled in with the type 'std::io::Error' when there 
were problems opening the file.

when you recognize situations in your code with multiple struct or enum 
definitions that differ only in the types of the values they hold, you can 
avoid duplication by using generic types instead.

//In Method Definitions

We can implement methods on structs and enums and use generic types in their
definitions, too. The code below shows the 'Point<T>' struct we defined with
a method named x implemented on it:
*/

//Implementing a method named x o the point<T> struct that will return a ref
//to the x field of type 'T'
/*
struct Point {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
*/
/*
Here, we've defined a method named x on 'Point<T>' that returns a
reference to the data in the field x.

Note that we have to declare 'T' just after 'impl' so we can ust 'T' to specify 
that we're implementing methods on the type 'Point<T>' By declaring 'T' as a 
generic type after 'impl', Rust can identify that we have chosen a different 
name for this generic parameter that the generic parameter declared in the
struct definition, but usig the same name is conventional. Methods written
with an 'impl' that declares the generic type will be defined on any instance
of the type, no metter what concrete tye ends up substituing for the generic type.

We can also specify contraints on generic types when defining methods on
the type. We could, for example, implement methods only on 'Point<f32>' instances
rather than on 'Point<T>' Instances with any generic type. Below, we use the 
concrete type 'f32', meaning we don't declare any types after impl.
*/

//An impl block that only applies to a struct with a particular concrete
//type for the generic type parameter T
/*
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
*/
/*
The code means that the type 'Point<f32>' will have a 'distance_from_origin'
method; other instances of 'Point<T>' where 'T' is not of type 'f32' will not 
have this method defined. The method measures how far our point is from the
point at coordinates (0.0, 0.0) and uses mathematical operations that are
available only for floating point types.

Generic type parameters in a struct definition aren't always the same as thos
you use in that same struct's method signatures. The code below uses the generic 
types X1 and Y1 for the 'Point' struct and x2 Y2 for the 'mixup' method
signature to make the example clearer. The methos creates a new 'Point' instance
with the x value from the 'self Pint' (of type x1) and the y value from the 
passed-in 'Point' (of type Y2).
*/

//A method that uses generic types different rom its struct's definition
/*
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p2.x, p3.y);
}
*/

/*
In 'main', we've defined a 'Point' that has an 'i32' for 'x' (with value 5)
and an 'f64' for 'y' (with value 10.4). the p2 variable is a 'Point' struct
that has a string slice for x (with value "Hello") and a 'char' for 'y' (with value 'c'). 
Calling 'mixup' on 'p1' with the argument 'p2' gives us 'p3', which will have
an 'i32' for 'x', becaue 'x' came from 'p1'. the 'p3' variable will have a 'char'
for 'y', because y came from 'p2'. the 'println!' macro call will print p3.x = 5,
p3.y = c'

The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with 'impl' and some are declared with the method
definition. Here, the generic parameters 'X1' and 'Y1' are declared fafter 'impl' because
they go with the struct definition. the generic parameters 'x2 and 'y2' are
declared after 'fn mixup' becaue they're only relevant to the method.
*/

//Performance of cose Using Generics

/*
You might be wondering whether there is a runtime cose when using generic
type parameters. the good news is that using generic types won't make your
program run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphixation of the code using
generics at compile time. Monomorphization is the process of turning 
generic code into specific code by filling in the conrete types that are used
when compiled. In this process, the compiler does the opposite of the steps
we used to create the generic function in the listing earler. The compiler loks 

at all the places where generic code is called and generates code for the
concrete types the generic code is called with.

Let's look at how this works by using the std library's generic 'Option<T>' enum:

let integer = Some(5);
let float = Some(5.0);

When Rust compiles this code, it performs monomorphization. During that process, 
the compiler reads the values that have been used in 'Option<T>' instances
and identifies two kinds of 'Option<T>: one is 'i32' and the other is 'f64'.
As such, it expands the generic definition of 'Option<T>' into two definitions 
specialized to 'i32' and 'f64', thereby replacing the generic definition
with the specific ones.

The monomorphized version og the code looks similar to the following:
*/

/*
*/
/*
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
*/

/*
the generic 'Option<T>' is replaced with the specific definitions created by the compiler. Because
Rust compiles generic code into that specifices the type in each instance, 
we pay no runtime cost for using generics. When the code runs, it performs
juast as it would if we had duplicated each definition by hand. The process of 
monomorphization makes Rust's generics extremely efficient at runtime.
*/

//Traits: Defining Shared Behavior

/*
A traight defines functionality a particular type has and can share with
other types. We can use traits to define shared behavior in an abstract way.
We can use trai bounds to specify that a generic type can be any type that 
has a certain behavior.

//Defining a trait

a type's behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those 
types. Trait definitions are a way to group method signatures together to 
define a set of behavior necessary to accomplish some purpose.

For example, let;s say we have multiple structs that hold various kinds and
amounts of text: a 'NewsArticle' struct that holds a news story filled in a 
particular location and a 'Tweet' that can have at most 280 characters along with
metadata that indicated whether is=t was a new tweet, a retweer, or a reply to
another tweet.

we are making a media aggregator library crate names 'aggregator' that can 
display summaries of data that might be stored in a 'NewsArticle' or 'Tweet' instance.
To do this, we need a summary from wach type, and we'll request that summart
by calling a 'summarize' methos on an instance. Below we see the definition of
 a public 'Summary' trait that expresses this behavior:
*/

//A summary that consists of the behavior provided by a 'summarize' methos
/*
fn main() {

}

pub trait Summary {
    fn summarize(&self) -> String;
}
*/

/*
Here, we declare a trait using the 'trait' keyword and then the triait's name
, which is 'Summary' in this case. We've also declared that the trait as 'pub'
so that crates depending on this crate can make use of this trait too, as we'll
see in a few examples. Inside the curly brackets, we declare the method signatures
that describe the behaviors of the types that implement this trait, which in
this case is 'fn summarize(s&self) -> String'

Afte the method signature, instead of providing an implementation within curly
brackets, we use a semicolo. Each type implementing this trait must 
provide its own custom behavior for the body of this method. The compiler will 
enforce that any type that has the 'Summary' trait will have the method 'Summarize'
defined with this signature exactly.

A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.

//Implementin a Trait on a Type

Now that we've defined the desired signatures of the 'Summar' trait's methods, 
we can implement it on the types in out media aggregator. The code below shows
an implementation of the 'Summary' trait on the 'NewsArticle' struct that uses
the headline, the authoe, and the location to create the return value of 'Summarize'
For the 'Tweet' struct, we define 'Summarize' as the username followed by the 
entire text of the tweet, assuming that tweet content is already limited to 280
characters.
*/

//Implementing the summary traight on the news article and tweet types
/*
fn main() {

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by{} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub rewteer: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/

/*
Implementing a trait on a type is similar to implementing regular methods.
The difference is that after 'impl', we put the trait name we want to implement, 
then use the 'for' keyword, and then specify the name of the type we want to 
implement the trait for. Within the 'impl block, we put the method signatures
that the trait definition has defined. Instead of adding a semicolon after each
signature, we use curly brackets and fill in the method body with the specific
behavior that we want to the methods of the trait to have for the particular
type.

Now that the library has implemented the 'summary' trait on 'NewsArticle' and 'Tweet'
, users of the crate can call the trait methods on instances of 'NewsArticle' and 'Tweet'
in the same way we call regular methods. the only difference is that the user
must bring the trait into scope as well as the types. He'res an example of how
a binary crate could use our 'aggregator' library crate:
*/

/*
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, aas you probably already know, people",
        ),
        reply: false,
        retweet: fasle,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
*/

/*
this code prints '1 new tweet: horse_ebooks: of course, as you probably
already know, people.'

Other crates that depend on the 'aggregatoe' crate can also bring the 'Summary' trait 
into scope to implement 'Summary' on their own types. One restrition to note is
that we can implement a trait on a type only if at least one of the trait or
the type is local to our crate. for example, we can implement std library traits like 
'Display' on a custom type like 'Tweet' as part of our 'aggregator' crate functionality, 
because the type 'Tweet' is local to our 'aggregator' crate. we can also implement
'Summary' on 'Vec<T>' i our 'aggregator' crate because the trait 'Summary' is
local to our 'aggregator' crate.

But we can't impement external traits on external types. For example, we can't
implement the 'Display' trait on 'Vec<T>' within our 'aggregator' crate, because 
'Display' and 'Vec<T>' are both defined in the std library and aren't local to our
'aggregator' crate. This restriction is a part of a property called coherence, 
and more specifically the orphan rule, so named because the parent type
without the rile, two crates implement the same trait for the same type, and 
Rust wouldn't know which implementation to use.

//Default implementations

Sometimes it's useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on  every type.
Then, as we implement the trait on a particular type, we can keep or override 
each method's default behavior.

Below, we specify a default string for the 'summarize' methos of the 'Summary' trait 
instead of only defining the method signature, as we did before.
*/

//Defining a summary trait with default implementation of the summarize method

/*
fn main() {

}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
*/

/*
To use a default implementation to summarize instances of 'NewsArticle', we specifically
we specify an empty 'impl' block with 'impl' 'Summary' for 'NewsArticle {}'.

Even though we're no longer defining the 'summarize' method on 'NewsArticle' directly
we've provided a default implementation and specified that 'NewsArticle' implements the 'Summary' trait.
As a result, we can still call the 'summarize' method on an instance of 'NewsArticle',
like this:
*/

/*
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PS, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The pittsburgh Penguins once again are the best \
          hockey team in the NHL>",
    ),
};

println!("New article available! {}", article.summarize());
*/

/*
Creating a default implementation doesn't require us to change anything about the 
implementation of 'Summary' on 'Tweet' in the example above. the reason is that the syntaz 
for overriding a default implementation is the same as the syntax for oimplementing
a trait method that doesn't have a default implementation.

Default implementations can call other methods in the same trait, even if those other
methods don't have a default implementation. In this way, a trait can provide a lot
of useful functionality and only require implementators to specify a small part of it
For example, we could define the 'Summary' trait to have a 'summarize_author'
method whose implementation is required, and then define a 'summarize' method that has
a default omplementation that calls the 'summarize_author' method:
*/

/*
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from the {}...)", self.summarizeauthor())
    }
}
 // to use this version of 'Summaru', w only need to define 'summarize_authos'
 //when we implement the trait on a type:

 impl Summary for Tweet {
    fn summarize_suthor(&self) -> String {
        format!("@{}", self.useramme)
    }
 }
 */

 /*
 After we define 'summarize_author' we can call 'summarize' on instances of the 'Tweet'
 struct, and the default implementation of 'summarize' will call the definition of 
 'summarize_author' tha we've provided. Because we've implemented 'summarize_author', the
 'Summary' trait has given us the behavior of the 'summarize' methos without 
 requiring us to write any more code.
*/

/*
fn main() {

}

let Tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people:,"
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
*/

/*
Note that it isn't possible to call the default implementation from an overriding
implementation of the same method.

//Traits as Parameters

Now that you know how to define and implement traits, we can explore hoe to use 
traits to define functions that accept many different types. We'll use the 'Summary' 
trait we implemented on the 'NewsArticle' and 'Tweet' types above to dfine a 'notify'
function that calls the 'summarize' methos on its 'item' parameter, which is 
of some type that implements the 'Summary' trait. To do this, we use the 'impl Trait' syntax
like this:

pub fn notify(item: &impl Summary) {
    println!("Breaking news!{}", item.summarize());
}

Instead of a concrete type of the 'item' parameter we specify the 'impl' keyword
and the trait name. this parameter accepts any type that implements the
specified trait. In the body of 'notify', we can call any methods on 'item' 
that come from the 'Summary' trait, such as 'summarize'. We can call 'notify' and 
pass in any instance of 'NewsArticle' or 'Tweet'. Code that calls the function with
any other type, sucha s a 'String' or an 'i32', won't compile becaue those
types don't implement 'Summary'

//Trait Bound Syntax

the 'impl' 'Trait' syntaz works for straightforward cases but is actually
syntax sugar for a longer form known as a trait bound; it looks like this:

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

this longer form is the equivalent to the example in the previous section
but is more verbose. We place trait bounds with the declaration of the 
generic type parameter after a colon and inside angle brackets.

the 'impl' 'trait' syntax is convenient and makes for more concise code
in simple cases, while the fuller trait bound syntax can express more complexity
in other cases, For example, we can have two parameters that implement 'Summary'. 
Doing so with the 'impl' 'Trait' syntax looks like this:

pub fn notify(item1: &impl Summary, item2: &impl Summary) {

Using 'impl' 'Trait is appropriate if we want this function to allow 'item1' and 
'item2' to have differnet types(as long as both types implement 'Summary'). If we 
want to force both paramethers to have the same type, howefer, we must use a 
trait bound, like this:

pub fn notify<TT: Summary>(item: &T, item2: &T) {

    The generic type 'T' specified as the type of the 'item1' and 'item2' parameters
    constrains the function such that the concrete type of the value passed
    as an argument for 'item1' and 'item2' must be the same.

    //Specifying Multiple Trait Bounds with the + Syntax

    We can also specify more than one trait bound. Say we wante to 'notify' to use display
    formatting as well as 'summarize' on 'itme': we specify n the 'notify' 
    definition that 'item' must implement both 'Display' and 'Summary'. We can 
    do so using the + syntax:

    pub fn notify(item &(impl Summary _ Display)) {

    the + syntax is lso valid with trait bounds on generic types:

    pub fn notify<T: Summary + Display>(item: &T) {

    With the two trait bounds specified, the body of 'notify' can call 'summarize' and 
    use {} to format item.

    //Clearer Trait Bounds with where Clauses

    Using too many trait bounds has its downsides. Each generic has its own trait
    bounds, so function swith multiple generic type parameters can contain
    lots of trait bound information between the function's name and its
    parameter list, making the function signature hard to read. For this reason,
    Rust has alternate syntax for specifying trait bounds inside a 'where' clause 
    after the function signature, like this:
*/
/*
    //where clause
    fn some_function<T, U>(t: &T, u: &U) -> i32where
    T: Display + Clone,
    U: Clone + Debug,
    {
*/
/*

//Returning Types that Implement Traits

We can also use the 'impl' 'Trait' syntax in the return position to return a value
of some type that implements a trait, as shown here:

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

By using 'impl' 'Summary' for the return type, we specify that the 'returns_summarizable'
function returns some type that implements the 'Summary' trait without
naming the concrete type. In this case, 'returns_summarizable' returns a 'Tweet', but the
code calling this fucntion doesn't need to know ehat.

Tha ability to specify a return type only by the trait it implements is especiallt
useful, in the context of closures and iterators. The 'impl' 'Trait' syntax lets you
concisely specify that a function returns some type that implements the 'Iterator'
trait without needing to write out a very long type.

However you can only use 'impl' 'Trait' if you're returning a singlt type.
For example, the code below returns wither a 'NewsArticle' or a 'Tweet' with the 
return type specified as 'impl' 'Summary' wouldn't work:

fn returns_summarizabl(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Champtionship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from()
            "The Pittsburgh Penguins once again are the best \
            hohckey team in the NHL.","
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

Returning either a 'NewsArticle' or a 'Tweet' isn't allowed due to restrictions
around how the 'impl' 'Trait' syntax is implemented in the compiler.

//Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an 'impl' block that uses generic type parameters, 
we can implement methods confditionally for types that implement the specified traits,
For example, the type 'Pait<T>' always implements the 'new' function to return
a new sintance of 'Pait<T>'. In the next 'impl' block, 'Pair<T>' only implements
the 'cmp_display' method if its inner type 'T' implemeents the ' PartialOrd' trait
that enables comparison and the 'Display' trait that enables printing.
*/
/*

//Conditionally Implementing Methods on a Generic Tyepe depending on trait bounds
use std::fmt::Display;

struct Pait<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if elf.x >= self.y {
            println!("The largest member is x ={}, self.x);")
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

We can also conditionally implement a trait for any type that implements another
trait. Implementations of a train on any type that satisfies the trait bounds are
called blanket implementations are are extensively used in the std library.
Fo r example, the std library implements the 'ToString' trait on any type that 
implements the 'Display' trait. The 'impl' block in the std lib looks similar to this
code:

impl<T: Display> ToString for T {

}

Because the standard library has this blanket implementation, we can call
the 'to_string' method defined by the 'ToString' trait on any type that implements
the 'Display' trait. For exaample, we can turn integers into their corresponding 
'String' values like this because integers implement 'Display':

let s = 3.to_string();

Blanket implementations appear in the documantation for the trait in the 
'Implementors' section.

Traits and trait bounds let us write code that uses generic type parameters to reduce
duplication but also specify to the compiler that we want the generic type
to have particular behavior. The compiler can then use the trait bound information
to check that all the concrete types used with our code provide
the correct behavior. In dynamically typed languages, we would get an error at 
runtime if we called a method on  a type which didn't define the method. But Rust
moves these errors to compile time so we're forced to fix the problems before our
code is even able to run. Additionally, we don't have to write code that checks for beha
-avior at runtime because we've already checked at compile time. Doing so improves
performance without having to give up the flexibility of generics.

// Validating Referenced with Lifetimes

Lifetimes are another kind of generic that we've already been using. rather
than ensuring that a type has the behavior we want, lifetimes ensure that the 
referenced are valid as long as we need them to be. Every reference in Rust has a lifetime, which
is the scope for which that reference is valid Most of the time, lifetimes are implcit
and inferred, just like most of the times, types are inferred. We only must annotate types
when multiple types are possible. In a similar way, we must annotate lifetimes
when the lifetimes references could be related in a few different way. Rust 
requires us to annotate the relationships using generic lifetime parameters
to ensure the actual reference used at runtime will definitely be
valid. 

Annotating lifetimes is not even a concept most other programming languages have, 
so this is going to feel unfamiliar. Although we won't cover lifetimes int their entirety
here, we'll discuss common ways you might encounter lifetime syntax so that 
you can get comfortable with the concept.

//Preventing Dangling References with Lifetimes

The main aim of lifetimes is to prevent dangling references, which cause a 
program to reference data other than the data it's intended to reference.
Consider the following code which has an inner and outer scope:

//An attempt to use a reference whose value has gone out of scope
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

The above example declares variables without giving them an intial valie **

The outer scope declares a variable names 'r' with noinitial value., and the
inner scope declares a variable named 'x' with the initial value of 5. Inside the 
inner scope, we attempt to set the value of 'r' as a reference to 'x'. Then the
inner scope ends, and we attempt to print the value in 'r'. This code won't compile
because the value 'r' is referring to has gone out of scope before we try to
use it. --run for error

The variable 'x' doesn't "live long enough." The reason is that 'x' will be out of
scope then the inner scope ends on "line 7". But 'r' is still valid for the outer scope;
because its scope is larger, we can say that i "liveslonger." If Rust allowed
this cod to work, 'r' would be referenceing memory that was deallocated when 'x'
went out of scope, and anything we tried to do with 'r' wouldn't work correctly.

Rust determines this invalid code  with a borrow checker.

//the Borrow Checker

The Rust compiler has a borrow checker that compares scopes to determine whether 
all borrow are valid. Below we see the same code as before, but with annotations
showing the lifetimes of the variables.

fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
 --just checks scope

Here, we've annotated the lifetime of 'r' with ''a' and the lifetime of 'x' with ''b'
As you can see, the inner ''b' block is much smaller than the outer ''a' lifetime block.
At compile time, Rust compares the size of the two lifetimes and sees that 'r' 
has a lifetime of ''a' but that it refers to memory with a lifetime of ''b'. The 
program is rejected because ''b' is shorter than ''a': the subject of the reference
doesn't live as long as the reference.

fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

--fixes the code from earlier (no dangles no errors)

Here, 'x' has the lifetime ''b', which in this case is larger than ''a'. This
means 'r' can reference 'x' because Rust knows that the reference in 'r' will always
be valid while 'x' is valid

Now that you know where the lifetimes of reference are and how Rust analiyzes
lifetimes, to ensure references will always be valid, let's explore generic lifetimes
of parameters and return values in the context of functions.

//Generic Lifetimes in Function

We'll write a function that returns the longer of two string slices. This function will
take two string slices and return a single string slice. After we've implemented the
'longest' fnuction, the code below should pront 'The longest string is abdc.'

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest (string1.as_str(), string2);
    println!("The longest string is {}", result);
}

Note that we want the function to take string slices, which are references, rather than strings, 
because we don't want the 'longest' function to take ownership of its parameters. 

If we try to implement the 'longest' function shown below, it won't compile:

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

--refer to errors

The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can't tell whether the refernce being returned refers to 'x' or 'y'.
Actually, we don't know either, because the 'if' block in the body of this function
returns a reference to 'x' and the 'else' block returns a reference to 'y'.

When we're defining this function, we don't know the concrete values that will be 
passed into this function, so we don't know whether the 'if' case or the 'else'
case will execute. We also don't know the concrete lifetimes of the references 
that will be passed in, so we can't look at the scopes as we did before.

The borrow checker can't determine this either, because it deoesn't know
how the lifetimes of 'x' and 'y' relate tot he lifetime of the return value. To fix
this error, we'll add generic lifetime parameters that define the relationship
between the reference so the borrow checker can perform its analysis.

//Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references live. Rather, they
describe the relationships of the lifetimes of multiple references to each other
without affecting the lifetimes. Just as functions can accept any type when the 
signature specifies a generic type parameterm functions can accept referenced
with any lifetime by specifying a generic lifetime parameter.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters
must start with an apostrophe(') and are usually all lowe=ercase and very short, like
generic types. Most people use the name ''a' for the fierst lifetime annotation. 
We place lifetime paramether annotations wafter the & of a reference, using
a space to separate the annotation from the reference's type.

Here are some examples: a reference to an 'i32' without a lifetime partner, a
reference to an 'i32' that has a lifetime parameter named ''a', and a mutable
reference to an '32' that also has the lifetime ''a'.

&i32    // a reference
&'a i32    // a reference with an explicit lifetime
&'a mut i32     // a mutable reference with an explicit lifetime

One lifetime annotation by itself deosn't have much meaning, because the annotations
are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other Let''s examine how the lifetime annotations relatie
to each other in the context of the 'longest' function.

//Lifetime Annotations in Function Signatures

to us lifetime annotations in function signatures, we need to declare the generic
lifetime parameters inside angle brackets between the function name and the parameter list,
just as we did with generic type parameters.

We want the signature to express the following constraint: the returned reference 
will be valid as long as both the parameters are valid. This is the relationship between
lifetimes of the parameters and the return value. We'll name the lifetime ''a' and
then add it to each reference, as shown below:

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

This code should compile and produce the result we want when we use it with
the 'main'function.

The function signature now tells Rust that for some lifetime 'a, the function takes
two parameters, both of which are sting sliced that live at least as long as
lifetime 'a. The function signature also tells Rust that the string slice returned
from the function will live at least as long as lifetime 'a'. In practive, it means
that the lifetime of the referred to by the function arguments. These relationships
are what we want Rust to use when analyzing the code.

Remember when we specify the lifetime parameters in this function signature, we've not
changing the lifetimes of any values passed in or returned. Rather, we're specifying
that the borrow checker should reject any values that don't adhere to these contraints.
Note that the 'longest' function doesn't need to know exactly how long 'x' and 'y' will
live, only that some scope can be substituted for 'a that will satisfy this signature.

When annotating lifetimes infunctions, the annotations go in the function signature,
not in the function body. The lifetime annotations become part of the contract of t
the function, much like the types in the signature. Having function signatures contain
the lifetime contract meants the analysis the Rust compiler does can be simpler.
If there's a problem with the way a function is annotated or the way it is called, the
compiler errors can point to the part of our code and the contraints more
precisely. If, instead, the compiler made more inferences about what we intended the relationships
of the lifetimes to be, the compiler might only be able to point to a use of our code
many steps away from the cause of the problem.

When we pass concrete referenced to 'longest', the concrete lifetime that is substituted
for 'a is the part of the scope of 'x' that overlapts with the scope of 'y'. In other
words, the generic lifetime 'a will get the concrete lifetime that is equal to the
smaller of the lifetimes of 'x' and 'y'. Because we've annotated the reurned reference
with the same lifetime parameter 'a, the retunred reference will also be valid 
for the length of the smaller of the lifetimes of 'x' and 'y'.

Let's look at how the lifetime annotations restrict the 'longest' unction by passing
inreferenced that have differenc concrete lifetimes.

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

In this example, 'string1' is valid until the end of the outer scope, 'string2' 
is valid until the end of the inner scope, ad 'result' referenced something that is 
valid until the end of the inner scope. Run this code, and you'll see that the borrow
checker approves;
It will compile and print 'Th longest string is long string is long'

Next let's try an example that shows what the lifetime of the reference in 'result'
must be the smaller lifetime of the two arguments. We'll move the declaration of the 'result'
variable outside the inner scope but leave the assignment of the value
to the 'result' variable inside the scope with 'string2'. Then we'll move the 'ptintln!'
tht uses 'reault' to outside the inner scope, after the inner scope has ended.

//Attempting to use result after strin2 has gone out of scope --won't compile
fn main() {
    let string1 -String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

will catch error when run aattempted

The error shows that for 'result' to be valud tfor the 'println!' statement, 'string2' would
need to be valid until the end of the outer scope. Tust knows this because we annotated the
lifetimes of the function parameters and return values using the same 
lifetime parameter 'a.

As humans, we can look at this code and see that 'string1' is longer than 'string2'
and therefore 'result' will ontain a reference to 'string1'. Because 'string1' has not gone
out of scope yet, a reference to 'string1;' wll still be valid for the 'println!' statement.
However, the compiler can'tsee that the reference is valid in this case. We've tolf
Rust that the lifetime of the references passd in.
Therefore, the borrow checker disallows the code above as possible having an invalid
reference.

Try designing more experiments that vary the values and lifetimes of the references
passed in tto the 'longest'
function and how the returned reference is used. Make hypotheses about whther or
not your experiments will pass the bowwo checker before you compile; then check for 
accuracy.

//Thinking in Terms of Lifetimes

the way in which you need to specify lifetime parameters depends on what your function is doing.
For example, if we changed the implementation of the 'longest' function to always
return the first parameter rather than the longest string slice, we wouldn't need
to specify a lifetime on the 'y' parameter. The following code will compile:

fn longest<'a>(c: &'a str, y: &str) -> &;a str {
    x
}

We've specified a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for tone of the paramters. If the reference
returned does not refer to one of the parametersm it must refer to a vlie will
go out of scope at the end of the function. consider the implementation below

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

Here, even though we've specified a lifetime parameter 'a for the return type, this 
implementation will fail to compile because the return value lifetime is not
related to the lifetime of the parameters at all

The problem is that 'result' goes out of scope and gets cleaned up at the end of the longest' function. We're also
trying to return a reference to 'result' from the function. There is no way we ccan
specify lifetime parameters that would change the dangling refreennce, and Rust won't 
let us create a dangling reference. In this case, the best fix would be to return
an owned data type rather than a reference so the calling function is
then responsible for cleaning up the value.

Ultimately, lifetime syntaz is about connecting the lifetimes of various 
parameters and return values of functions. Once they're connected, Rust has enough 
information to allow memory-safe operations and disallow operations
that would create dangling pointers or otherise violte memory safety.

//Lifetime Annotations in Struct Definitions

So far, the structs we've defined all hold owned types. We can define structs 
to hold references,
but in that case we would need to add a lifetime annotation on every reference in 
the struct's definition. Below is a struct names 'ImportantExcerpt' which
holds a string slice:

//A struct that holds a reference, requireing a lifetime annotation
struct Importantexcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some yeard ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i - ImportantExcerpt {
        part: first_sentence,
    };
}

This truct has the ingle field 'part' that hods a string slice, which is a reference.
As with generic data types, we declare the name of the generic lifetime parameter
inside angle brackets after the name of the struct so we can use the
lifetime parameter in th ebody of the struct definition. This annotation means an
instance of 'ImportantExcerpt' can't outlive the reference it holds in its
'part field'.

The 'main' function here created an instance of the 'ImportantExcerpt' struct that 
holds a reference to the first sentence of the 'String' owned by the vriable 'novel'. doens't
goe out of scope until after the 'ImportantExcerpt' goes out of scope, so ther 
reference in the 'ImportantExcerpt' instance is valid.

//Lifetime Ellison

you've learned that every reference has a lifetime and that you need to specifythe life
time parameters for functions or structs that use references, However we have had
functions that compiled without lifetime annotations.

dn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' '{
            return &s[0..i];
        }
    }

    &[..]
}

The reason this function compiles without lifetime annotations is historical: in early 
versions of Rust it wouldn't have compiled beause every reference needed an explicit lifetime
at that time the function signature would have been written like:

fn first_word<'a>(s: &'a str) -> &'a tr {

}

The patterns programmed into Rusts's analysis of referenced are called the lifetime
elison rules. These aren't rules for programmers to follow; theyre a set of particular
cases that the compiler will consider, and if your code fitst hese cases you don't need
to write the lifetimes explicitly.
The ellison rules don't provide full inference. If Rust deterministically applies
the rules but there is still ambiguity as to what lifetimes the referenced
have, the compiler won't guess what the lifetime of the remaining refrerenced should be
Instead of guessing, the compiler will give you an error that you
can resolve by addidng the lifetime annotations.

Lifetimes on function or method parameters are called input lifetimes, and lifetimes
on return values are called output lifetimes.

The compiler uses three rules to figure out the lifetimes of the references 
when there aren's explicit annotations. Ther first rule applies to input
lifetimes, and the second and thirs rules apply to output lifetimes. If the compiler
gets to the end of the three rules and there are still referenced for which
it can't figure out lifetimes, the compiler will stop with an error.
These rules apply to 'fn' definitions as well as 'impl' blocks.

The fist rules is that the compiler assigns a lifetime parameter to each paameter
that's a reference. Inother words, a function with one parameter gets one lifetime
parameter: 'fn fooM'a>(x: &;a i32); a function with two parameters gets two separate
lifetimes parameters: 'fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on

the second rule is that, if there is exactly one input lifetime parameter, that lifetime
is assigned to all output lifetime parameters: 'fn foo<'a>(x: &'a i32) -> &'a i32'.

The third rule is that, if there are multiple input lifetime parameters, but one 
of them is '&self' or '&mut self' becaue this is a method, the lifetime of 'self'is assigned
to all output lifetime parameters. This third rule makes methods much nicer to 
read and write because fewer symbols are becessary.

The signature starts without any lifetimes associated with the references:
fn first_word(s: &str) -> &str {

}

then the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We'll call it 'a as usual, so now the sig is like this:

fn first_word<'a>(s: &;a str) -> &str {

}

Then the second rule applies because there is exactly one input lifetime. The second rule
specifies that the lifetime of the one input prarameter
gets assigned to the output lifetime, so the signature is now this:

fn fist_word<'a>(s: &'a str) -> &'a str {

}

Now all the references in this function signature have lifetimes, and the compiler
can continue its analysis without needing the programmer to annotate the
lifetimes in this functions signature

this function had no lifetime parameters when we started

fn longest(x: &str, t: &str) -> &str {

}

This time we have two parameters instead of one, so we have two lifetims:

fn lonfest<'a, 'b>(x: &;a str, y: &'b str) -> &str {

}

You can see that the second rule doesn't apply because there is more than one input lifetime
The thrid rule doesn't apply ithe, because 'longest' is a function tahter than a method, 
so none of the paramethers are 'self'. After working through all three rules, we still
haven't figured out what the retutn type' lifetime is. This is why we got an
error trying to compile all thecode in the earlier example

Because the third rule really only applies in method signatures, we'll look at lifetimes
in that context to see why the hird rule means we don't have to annotate lifetimes
in method signatures very ofthen.

//Lifetime Annotations in Method definitions

When we implement methods   on a truct with lifetimes, we use the same syntax 
as that of generic type parameters. Where we declare and use the lifetime
parameters depends on whether they're related to the truct fels or the method parameters
and return values.

Lifetime names for striuct firsld alwyays need to be declared after the 'impl' keyword
and then used after the struct's name, because those lifetimes are part of
the struct's type.

In method signatures inside the 'impl' block, referenced might be ties to the lifetime
of referenced in the struct's firelds, or they might be indepeendent. In addition, the 
lifetimes elison rules often make t so that lifetime annotations aren't necessary
in method signatures. Let's look at some examples

First we'll use a method named 'level' whose only parameter is a reference to 'self'
and whose return value is an 'i32', which is not a ref to anything:
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

The lifetime parameter declaration after 'impl' and its use after the type name are
required, but we're not required to annotate the lifetime of the reference to 'self'
because of the first elison rule.

here's an example of the 3rd rule:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please: {}", announcement);
        self.part
    }
}

There are two input lifetimes, so Rust applies the first lifetime ellison rule and
gives both &self and announcement theor own lifetimes. Then, because one of the parameters
is '&self', the return type gets the lifetime of &self, and all the lifetimes have
been accounted for.

//The static Lifetime

One special lifetime we need to discuss is 'statis', which denote that the affected
reference can live for the entire duration of the program. all string literals
have the 'static' lifetime, which we can annotate as follows:
let s: &'static str = "I have a static lifetime,";

The ext of this string is trored directly in the program's binary, which is always available.
therefore the lifetime of all string literals is 'statis'

You might see suggestions to sue 'statis' lifetime in error messages but before 
specifying 'static' as the lifetime for a reference, think abou whether the reference 
you hvave made actually lives the entire lifetime of your program or not. and whether
you cant it to. Most of the time , an error message suggesting the 'static' lifetime 
results from attempting to create a dangling reference or a mismatche of the available
lifetimes. Isuch cases, the solution is fixing those problems, not specifying the 'static'
lifetime

//Generic Type Parameters, Trait bounds, and lifetimes, together

use std::fmt::Display;

fn longest_with_an_announcement<;a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
T: Display, 
{
    println!("annoucnement! {}, ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

Thhis is the longest function from above that returns the longer of the two string 
slices. But now ihas an extra parameter named 'ann' pf the generic type 'T' which can be 
filled in by any type that implements the 'Display' trait as spcified by the 'where'
clasue This extra parameter will be printed using {}, which is why the 'Display' trait bound is necessary/ Because lifetimes
are a type of generic, the declerations of the lifetime parameter 'a
and the generic tpye parameter 'T' o in the same list inside the angle brackets after
the fucntion name.