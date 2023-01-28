fn main() {
    println!("Hello, world!");
}

/*
Vec<T> = vector : Vec = vector <T> = type in the vector
Vectors allow you to store more than one value in a single data structure
that puts all the values next to each other in memory. They can only store
values of the same type. They're useful when you have a list of items, such
as the lines of text in a file or the prices of items in a shopping cart.
*/

// Creating a new, empty vector to hold values of type i32
let v: Vec<i32> = Vec::new();

//Creating a new vector containing values
let v = vec![1, 2, 3];

//Using the push method to add values to a vector
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

/*
The numbers we place inside are all of type i32, and Rust infers this from
the data, so we don't need the Vec<i32> annotation.
*/

//Reading Elements of Vectors
/*
There are two ways to reference a value stored in a vector: via indexing
or using the 'get' method. Refer to the examples below. 
*/

//Using indexing syntax or the 'get' method to access an item in a vector
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None =. println!("There is no third element."),
}

//Attempting to access the element at index 100 in a vector containing five elements
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let  does_not_exist = v.get(100);

/*
When we run this code, the first [] method will cause the program to panic 
because it references a nonexistent element. This method is best used when
you want your program to crash if there's an attempt to access an element
past the end of the vector.
*/

//Attempting to add an element to a vector while holding a reference to an item
let mut v =vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");

//Iterating over the Values in a Vector
/*
To access each element in a vector in turn, we would iterate through all
of the elements rather than use indicies to access one at a time. The 
example below shows how to use a 'for' loop to get immutable references to
each element in a vector of i32 values and print them.
*/

//Printing each element in a vector by iterating over the elements using
//a 'for' loop.
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

//Iterating over mutable references to elements in a vector
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

/*
To change the cvalue that the mutable reference refers to, we have to use
the '*' dereference operator to get to the value in 'i' before we can use
the += operator.

Iterating over a vector is safe because of the borrow checker's rules.
The reference to the vector that the 'for' loop holds prevents simultaneous
modification of the whole vector.
*/

//Using an Enum to Store Multiple Types
/*
Vectors can only store values that are the same type. 
The variants of an enum are defined under the same enum type, so when we 
need one type to represent elements of different types, we can define and 
use an enum!

We can define an enum whose variants will hold the different value types, and
all the enum variants will be considered the same type: that of the enum.
We can the create a vector to hold that enum and so, ultimately, holds
different types.
*/

//Defining an enum to store values of different types in one vector
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

/*
Rust needs to know what types will be in the vector at compile time so it
knows exactly how much memory on the heap will be needed to store each element.
We must be explicit about what types are allowed in this vector, because if
Rust allowed a vector to hold any type, there would be a chance that one or
more of the types would cause errors with the operations performed on the
elements of the vector.

Using an enum plus a 'match' expression means that Rust will ensure at compile
time that every possible case is handled.

The enum technique won't work if you don't know the exhaustive set of types a
program will get at runtime to store in a vector. In that case, use a trait
object instead.
*/

//Dropping a Vector Drops Its Elements
//Like any other 'struct', a vector is freed when it goes out of scope.

//Showing where the vector and its elements are dropped
{
    let v = vec![1, 2, 3, 4];

    //do stuff with v
} // <- v goes out of scope and is freed here


//Creating a New String
/*
Many of the same operations available with 'Vec<T>' are available with 'String'
as well, because 'String' is actually implemented as a wrapper around a vector
of bytes with some extra guarantees, restriction, and capabilities. An example
of a function that works the same way with 'Vec<T>' and 'String' is the 'new'
function to create an instance, shown below.
*/

//Creating a new empty string
let mut s = String::new();
/*
with this line, we create a new empty string called 's', which we  can load 
data into. When we want to start the string with some initial data, we use
the 'to_string' method. This method is available on any type that implements
the 'Display' trait, as string literals do.
*/

//Using the 'to_string' method to create a 'String' from a string literal
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
/*
This creates a string containing 'initial contents'. We can also use the
function 'String::from' to create a 'String' from a string literal.
*/

//Using the String::from function to create a 'String' from a string literal
let s = String::from("initial contents");

/*
Strings are UTF-8 encoded, so we can include any properly encoded data in them.

All of the below examples are 'String' values.
*/

//Storing greetings in different languages in strings
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

//Updating a String

/*
A 'String' can grow in size and its contents can change, just like the contents
of a Vec<T>, if you push more data into it. In addition, you can conveniently
use the + operator or the 'format!' macro to concatenate 'String' values.
*/

//Appending to a String with push_str and push
//
// we can grow a 'String' by using the 'push_str' method to append a string
//slice, as shown below.

//Appending a string slice to a 'String' using the 'push_str' method
let mut s = String::from("foo");
s.push_str("bar");

/*
After these two lines, 's' will contain 'foobar'. The 'push_str' method takes
a string slice because we don't necessarily want to take ownership of the 
parameter. We want to be able to use 's2' after appending its contents to 's1'.
*/

//Using a string slice after appending its contents to a 'String'
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");

/*
If the 'push_str' method took ownership of 's2', we wouldn't be able to print
its value on the last line. However, this code works as we'd expect.

The 'push' method takes a single character as a parameter and adds it to the
'String'. The example below adds the letter "l" to a 'string' using the 'push'
method.
*/

//Adding one character to a 'String' value using 'push'
let mut s = String::from("lo");
s.push('l');

//as a result, 's' will contain 'lol'

//Concatenation with the + Operator or the format! Macro

//One way to combine two existing strings is to use the '+' operator.

//Using the '+' operator to combine two 'String' values into a new 'String' value
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

/*
The string 's3' will contain 'Hello, world!'. The reason 's1' is no longer 
valid after the addition, and the reason we used a reference to 's2', has to
do with the signature of the method that's called when we use the + operator.
The + operator uses the 'add' method, whose signature looks something like:

fn add(self, s: &str) -> String {

}

In the std library, you'll see 'add' defined using generics and associated
types. Here, we've substituted in concrete types, which is what happens
when we call this method with 'String' values. The above signature gives us
the clues we need to understand the tricky bits of the + operator.

First, s2 has an &, meaning that we’re adding a reference of the second 
string to the first string. This is because of the s parameter in the add 
function: we can only add a &str to a String; we can’t add two String values 
together. But wait—the type of &s2 is &String, not &str, as specified in the 
second parameter to add.

The reason we’re able to use &s2 in the call to add is that the compiler can 
coerce the &String argument into a &str. When we call the add method, Rust 
uses a deref coercion, which here turns &s2 into &s2[..]. We’ll discuss 
deref coercion in more depth in Chapter 15. Because add does not take 
ownership of the s parameter, s2 will still be a valid String after this 
operation.

Second, we can see in the signature that add takes ownership of self, 
because self does not have an &. This means s1 will be moved into the add 
call and will no longer be valid after that. So although let s3 = s1 + &s2; 
looks like it will copy both strings and create a new one, 
this statement actually takes ownership of s1, appends a copy of the contents 
of s2, and then returns ownership of the result. In other words, it looks 
like it’s making a lot of copies but isn’t; the implementation is more 
efficient than copying.

If we need to concatenate multiple strings, the behavior of the + operator
gets unwueldy:

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe"); 

let s = s1 + "-" + &s2 + "-" + &s3;

At this point, 's' will be 'tic-tac-toe'. With all of the + and " characters,
it's difficult to see what's going on. For more complicated string combining,
we can instead use the 'format!' macro:

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");

This code also sets s to tic-tac-toe. The format! macro works like println!, 
but instead of printing the output to the screen, it returns a String with 
the contents. The version of the code using format! is much easier to read, 
and the code generated by the format! macro uses references so that this call 
doesn’t take ownership of any of its parameters.

//Indexing into Strings

//You'll get an error if you try to access parts of a string using indexing 
//syntax in Rust.

//Attempting to use indexing syntax with a String
let s1 = String::from("hello");
let h = s1[0];

We can see that Rust strings don't support indexing, which has to do with how 
Rust stores strings in memory.

//Internal Representation

A 'String' is a wrapper over a 'Vec<u8>'. Some examples below.

let hello = String::from("Hola");

In this case, 'len' will be 4, which means the vecot stoing the string "Hola"
is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8.

let hello = String::from("Здравствуйте");

This takes 24 bytes to encode in UTF-8 because each Unicode scalar value in
that string takes 2 bytes of storage. Therefore, an index into the string's
bytes will not always correlate to a valid Unicode scalar value.

//Bytes and Scalar Values and Grapheme Clusters! Oh My!

Another point about UTF-8 is that there are actually three relevant ways to 
look at strings from Rust’s perspective: as bytes, scalar values, and 
grapheme clusters (the closest thing to what we would call letters).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, 
it is stored as a vector of u8 values that looks like this:

[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
That’s 18 bytes and is how computers ultimately store this data. 
If we look at them as Unicode scalar values, which are what Rust’s char type 
is, those bytes look like this:

['न', 'म', 'स', '्', 'त', 'े']
There are six char values here, but the fourth and sixth are not letters: 
they’re diacritics that don’t make sense on their own. 
Finally, if we look at them as grapheme clusters, we’d get what a person 
would call the four letters that make up the Hindi word:

["न", "म", "स्", "ते"]
Rust provides different ways of interpreting the raw string data that 
computers store so that each program can choose the interpretation it needs, 
no matter what human language the data is in.

A final reason Rust doesn’t allow us to index into a String to get a 
character is that indexing operations are expected to always take constant 
time (O(1)). But it isn’t possible to guarantee that performance with a 
String, because Rust would have to walk through the contents from the 
beginning to the index to determine how many valid characters there were.

//Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the 
return type of the string-indexing operation should be: a byte value, 
a character, a grapheme cluster, or a string slice. If you really need to 
use indices to create string slices, therefore, Rust asks you to be more 
specific.

Rather than indexing using [] with a single number, you can use [] with a 
range to create a string slice containing particular bytes:

let hello = "Здравствуйте";

let s = &hello[0..4];
Here, s will be a &str that contains the first 4 bytes of the string. 
Earlier, we mentioned that each of these characters was 2 bytes, which means 
s will be Зд.

If we were to try to slice only part of a character’s bytes with something 
like &hello[0..1], Rust would panic at runtime in the same way as if an 
invalid index were accessed in a vector:

//Methods for Iterating Over Strings
The best way to operate on pieces of strings is to be explicit about whether 
you want characters or bytes. For individual Unicode scalar values, use the 
chars method. Calling chars on “Зд” separates out and returns two values of 
type char, and you can iterate over the result to access each element:

for c in "Зд".chars() {
    println!("{c}");
}
This code will print the following:

З
д
Alternatively, the bytes method returns each raw byte, which might be 
appropriate for your domain:

for b in "Зд".bytes() {
    println!("{b}");
}
This code will print the four bytes that make up this string:

208
151
208
180
But be sure to remember that valid Unicode scalar values may be made up of 
more than 1 byte.

Getting grapheme clusters from strings as with the Devanagari script is 
complex, so this functionality is not provided by the standard library. 


//Storing Keys with Associated Values in Hash Maps

The type 'HashMap<K, V> stores a mapping of keys of type 'K' to values of 
type "V" us a hashing function, which determined how it places these keys
and values into memory.

Hash maps are useful when you want to look up data not by using an index,
as you can with vectors, but by using a key that can be of any type.

//Creating a New Hash Map

One way to create an empty hash map is using 'new' and adding elements with
'insert'. See the example below for a demonstration.

//Creating a new hash map and inserting some keys and values
use std::collections::HashMap;

let mu scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

There are no built-in macros to construct hashmaps.

Just like vectors, hash maps store their data on the heap. This 'HashMap' has 
keys of type 'String' and values of type 'i32'. Like vectors, hash maps are
homogeneous: all of the keys must have the same type as each other, and all
of the values must have the same type.

//Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the 'get' method
as shown below.

//Accessing the score for the Blue team stored in the hash map
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

Here, score will have the value that’s associated with the Blue team, 
and the result will be 10. The get method returns an Option<&V>; 
if there’s no value for that key in the hash map, get will return None. 
This program handles the Option by calling copied to get an Option<i32> 
rather than an Option<&i32>, then unwrap_or to set score to zero if scores 
doesn't have an entry for the key.

We can iterate over each key/value pair in a hash map in a similar manner 
as we do with vectors, using a for loop:

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}

This will print each pair in an arbitrary order:

//Hash Maps and Ownership

For types that implement the Copy trait, like i32, the values are copied 
into the hash map. For owned values like String, the values will be moved 
and the hash map will be the owner of those values.

//Showing that keys and values are owned by the hash map once they're inserted
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
//field_name and field_value are invalid at this point, try using them and
//see what compiler error you get

We aren't able to use the variables 'field_name' and 'field_value' after they've
been moved into the hash map with the call to 'insert'.

If we insert references to values into the hash map, the values won't be moved
into the hash map. The values that the references point to must be valid for
at least as long as the hash map is valid.

//Updating a Hash Map

Although the number of key and value pairs is growable, each unique key can 
only have one value associated with it at a time.

When you want to change the data in a hash map, you have to decide how to
handle the case when a key already has a value assigned. You could replace the
old value with the new value, completely disregarding the old value. You could
keep the old value and ignore the new value, only adding the new value if the
key doesn't already have a value. You could also combine the old value and the
new value.

//Overwriting a Value

If we insert a key and a value into a hash map and then insert that same key
with a different value, 
the value associated with that key will be replaced. Even though the example 
below calls 'insert' twice, the hash map will only contain one key/value pair
because we're inserting the value for the Blue team's key both times.

//Replacing a value stored with a particular key
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);

This code will print {"Blue": 25}. The original value of '10' was overwritten.

//Adding a Key and Value Only If a Key Isn't Present

It's common to check whether a particular key already exists in the hash map
with a value then take the following actions: if the key does exist in the 
hash map, the existing value should remain the way it is. If the key doesn't
exist, insert it and a value for it.

Hash maps have a special API for this called 'entry' that takes the key you
want to check as a parameter. The return value of thie 'entry' method is an
enum called 'Entry' that represents a value that might or might not exist.
Let's say we want to check whether the kwy for the Yellow team has a value
associated with it. If it doesn't, we want to insert the value 50, and the 
same for thr Blue team. Using the 'entry' API is demonstrated in the code 
below:

//Using the entry method to only insert if the key doesn't already have a value
use std::collections::HashMap;

let mut scores = HashMap::new();
scored.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

The 'or_insert' method on 'Entry' is defined to return a mutable reference to
the value for the corresponding 'Entry' key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the above code block will print: {"Yellow": 50, "Blue": 10}

The first call to 'entry' will insert the key for the Yellow team with the
value 50 because the Yellow team doesn't have a value already. The second call
to 'entry' will not change the hash map because the Blue team already has the
value 10.

//Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key's value and then
update it based on the old value. For instance, the code bewlow counts how 
many times each word appears in some text. We use a hash map with the words
as keys and increment the value to keep track of how many times we've seen
that word. If it's the first time we've seen a word, we'll first insert the
value 0.

//Counting occurences of words using a hash map that stores words and counts
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);

This code will print {"world": 2, "hello": 1, "wonderful": 1}.
You might see the same key/value pairs printed in a different order: iterating
over a hash map happens in an arbitrary order.

The 'split_whitespace' method returns an iterator over sub-slices, separated
by whitespace, of the value in 'text'. The 'or_insert' method returns a mutable
reference(&mut V) to the value for the specified key. Here we store that
mutable reference in the 'count' variable, so in order to assign to that
value, we must first dereference 'count' using the asterisk(*). The mutable
reference goes out of scope at the end of the 'for' loop, so all of these
changes are safe and allowed by the borrowing rules.

//Hashing Functions

By default, 'HashMap' uses a hashing function called SipHash that can provide
resistance to Denial of Service(DoS) attacks involving hash tables. This is not
the fastest hashing algorithm available, but the trade-off for better security
that comes with the drop in performance is worth it. If you profile your code
and find that the default hash function is too slow for your purposes, you 
can switch to another function by specifiying a different hasher. A hasher is
a type that implements the 'BuildHasher' trait. Crates.io has libraries shared 
by other Rust users that provide hashers implementing many common hashing
algorithms.


//Summary

Vectors, strings, and hash maps will provide a large amount of functionality 
necessary in programs when you need to store, access, and modify data. 
Here are some exercises you should now be equipped to solve:

Given a list of integers, use a vector and return the median (when sorted, 
    the value in the middle position) and mode (the value that occurs most 
        often; a hash map will be helpful here) of the list.
Convert strings to pig latin. The first consonant of each word is moved to 
the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words 
that start with a vowel have “hay” added to the end instead (“apple” becomes 
    “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to 
Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of 
all people in a department or all people in the company by department, 
sorted alphabetically.
