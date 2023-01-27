
// A front_of_house module containing other modules that then contain functions
/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

/* 
Inside modules, we can place other modules, as in this case with the modules
'hosting' and 'serving'. Modules can also hold definitions for other items, 
such as structs, enums, constants, traits, and functions -as displayed above-


Using modules, we can group related definitions together and name why they're
related. This makes the code navigable by groups rather than reading every
definition, which makes it easier to find the relevant definitions.\

So, when adding new functionality to the code, you'll know where to place
your addition and keep the program organized.

//Paths for Referring to an Item in the MOdule Tree

To show Rust where to find an item in a module tree, we use a path in the
same way we would navigate a filesystem. To call a function, we need to 
know its path.

A path can take two forms:

an absolute path is the full path starting from a crate root; for code from
an external crate, the absolute path begins with the crate name, and for
code from the current crate, it starts with the literal 'crate'.

a relative path starts from the current module and uses 'self', 'super', or 
an identifier in the current module.

both types of paths are followed by one or more identifiers separated by
double colons (::).
*/

//Calling the add_to_waitlist function using absolute and relative paths
/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

/*
In Rust, all items(functions, methods, structs, enums, modules, and constants)
are private to parent modules by default. If you want to make an item like 
a function or struct private, you put it in a module.

Items in a parent module can't use the private items inside child modules,
but items in child modules can use the items in their ancestor modules.
Child modules wrap and hide their implementation details, but the child modules
can see the context in which they're defined.

The module system was chosen to function this way so that hiding inner
implementation details is the default and you know which parts of the inner
code you can change without breaking outer code. This is optional with 'pub'
keyword, which is used to make an item public.
*/

//Exposing Paths with the pub Keyword 
/*
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

/*
Making a module public doesn't make its content public. The 'pub' keyword
on a module only lets code in its ancestor modules refer to it, not access
its inner code. Because modules are containers, it doesn't do much for us to
only make the module public; we need to make one or more items within the
module public as well.
*/

//Adding the 'pub' keywork to mod hosting and fn add_to_waitlist allowing fn call
//from eat_at_restaurant
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

//Calling a function using a relative path starting with 'super'
/*
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
*/

/*
Since the back_of_house module and the deliver_order function are likely
to stay in the same relationship to each other and get moved together should
we decide to reorganize the crate's module tree. We used 'super' so we'll have 
fewer places to update code in the future if this code gets moved
to a different module.
*/

//Making Structs and Enums Public

/*If we use 'pub' before a struct definition, we make the struct public, but
the struct's fields will still be private. We can make each field public
or not on a case by case basis. In the example below we've defined a public
'back_of_house::Breakfast' struct with a public 'toast' field but a private
'seasonal_fruit' field. A scenario where this example would apply is where
the customer could pick which type of bread comes with thir meal, but the chef
decides which fruit accompanies the meal based on what's in season and in stock.
The available fruit changes quickly, so customers can't choose the fruit
or even see which fruit they'll get.
*/

//A struct with some public fields and some private fields
/*
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
     meal.seasonal_fruit = String::from("blueberries");
}
*/

/*
In contrast, if we make an enum public, all of its variants are then public.
We only need the 'pub' before the 'enum' keyword as shown in the example below.
*/

//Designating an enum as public makes all its variants public
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

/*
Because we made the 'Appetizer' enum public, we can use the 'Soup' and 'Salad'
variants in eat_at_restaurant.

Enums aren't useful unless their variants are public; if enum variants weren't 
public by default, we would have to annotate all enum variants with 'pub' in 
every case.

Structs are useful without public fields, so struct fields follow the general
rule of being private by default.
*/

//Bringing Paths into Scope with the use Keyword

/*
We can create a shortcut to a path with the 'use' keyword once, and then
use the shorter name everywhere else in the scope.

In the example below, we bring the 'crate::front_of_house::hosting' module
into the scope of the 'eat_at_restaurant' function so we only have to specify
'hosting::add_to_waitlist' to call the 'add_to_waitlist' function 
in 'eat_at_restaurant'.
*/

//Bringing a module into scope with 'use'
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

/*
Paths brought into scope with 'use' also check privacy, like any other paths.

'use' only creates the shortcut for the particular scope 
in which the 'use' occurs. Below shows the shortcut used out of scope.
*/

//A use statement only applies in the scope it's in --will throw an error
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/

//Creating Idiomatic use Paths

//Bringing the add_to_waitlist functino into scope with use, 
//which is unidiomatic
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
*/

/*
Although this accomplishes the same task as the previous example, the previous
example is the idiomatic way to bring a function into scope with use.
Bringing the function's parent module into scope with use means we have to 
specify the parent module when calling the function. Specifying the parent
module when calling the function makes it clear that the function isn't
locally defined while still minimizing repetition of the full path. 
In the example above, it's unclear as to where add_to_waitlist is defined.

Wehn bringing in structs, enums, and other items with 'use', it's idiomatic
to specify the full path. The example below shows the idiomatic way to bring
the standard library's 'HashMap' struct into the scope of a binary crate.
*/

//Bringing 'HashMap' into scope in an idiomatic way
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/

/*
The exception to this idiom is if we're bringing two items with the same name
into scope with 'use' statements, because Rust doesn't really allow that.

The example below shows how to bring two 'Result' types into scope that have
the same name but different parent modules and how to refer to them.
*/

//Bringing two types with the same name into the same scope requires
//using their parent modules
/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {

}
*/

/* 
Using parent modules distinguishes the two 'Result' types. Otherwise, 
we would have two 'Result' types in the same scope and Tust wouldn't know
which one we meant when we used 'Result'.
*/

//Providing New Names with the 'as' Keyword
/*
Another way to mitigate problems when bringing two types of the same name into
the same scope is specifiying 'as' and a new local name, or alias, after
the path for the type. The example below shows another way to write the code
from the previous example by renaming onw of the two 'Result' types using 'as'.
*/

//Renaming a type when it's brought into scope with the 'as' keyword
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {

}

fn function2() -> IoResult<()> {

}
*/

/*
In the second 'use' statement, we chose a new name for the type, which won't
conflict with the 'Result' from 'std::fmt' that we've also brought into scope.
*/

//Re-exporting Names with pub use
/*
When we bring a name into scope with the 'use' keyword, the name available
in the new scope is private. To enable the code that calls our code to refer
to that name as if it had been defined in that code's scope, we can combine
'pub' and 'use'. This is called 're-exporting' because we're bringing an item
into scope but also making that item available for others to bring into their scope.
*/

//Making a name available for any code to use from a new scope with pub use
/*
mod fronnt_of_house {
    pub mod hosting {
        pub fn ass_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

/*
Before this change, external code would have to call the 
add_to_waitlist function by using the path 
restaurant::front_of_house::hosting::add_to_waitlist(). 
Now that this pub use has re-exported the hosting module from the root module, 
external code can now use the path restaurant::hosting::add_to_waitlist() 
instead.

This is useful when the internal structure of your code is different from how 
programmers calling your code would think about the domain. With 'pub use',
we can write our code with one structure but expose a different structure.
When we do this, our library becomes well organized for programmers calling
and working on the library. 
*/

//Using External Packages

/*
Pulling any external crates into your package involves the following steps: 
Listing them in your package's 'Cargo.toml' file
Using 'use' to bring items from their crates into scope.

Even thought the std library is shipped with Rust, we still need to refer 
to it with 'use' to bring items from there into our package's scope.

For example, with 'HashMap' we would use this line:
use std::collections::HashMap;

This is an absolute path starting with std, the name of the std library crate.
*/

//Using Nested Paths to Clean Up Large use Lists
/*
While using multiple items defined in the same crate or same module, listing 
each item on its own takes up a lot of verticle space. For example:
use std::cmp::Ordering;
use std::io;

can utillize nested paths to bring these items into scope in one line.
Refer to the example below.

//Specifying a nested path to bring multiple items with the same prefix into scope
use std::{cmp::Ordering, io};

We can use a nested path at any level in a path, which is useful when combining
two 'use' statements that share a subpath. 

//Two 'use' statements where one is a subpath of the other
use std::io;
use std::io::Write;

//Combininf the paths into one 'use' statement --using 'self' in nested paths
use std::io::{self, Write};

This line brings std::io and std::io::Write into scope.
*/

//The Glob Operator
/*
If we want to bring all public items defined in a path into scope,
we can specify that path followed byt the * glob operator:

use std::collections::*;

This 'use' statement brings all public items defined in 'std::collections' into
the current scope. Use caution, as Glob can make it harder to tell what names
are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test
into the 'tests' module;
The glob operator is also sometimes used as part of the prelude pattern.
*/

//Separating Mosules into Different Files
/*
Here, we'll extract modules into files instead of having all the modules defined
in the crate root file.

//Declaring the front_of_house module whose body will be in src/front_of_house.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//Definitions inside the front_of_house module in src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

You only need to load a file using a mod declaration once in your module tree.

//Extracting hosting module to its own file

//change src/front_of_house.rs to contain only the declaration of the 'hosting'
//module
pub mod hosting;

//create a src/front_of_house directory and a file hosting.rs to contain the
//definitions made in the 'hosting' module
pub fn add_to_waitlist() {}

We’ve moved each module’s code to a separate file, and the module tree 
remains the same. The function calls in eat_at_restaurant will work without 
any modification, even though the definitions live in different files. 
This technique lets you move modules to new files as they grow in size.

Note that the pub use crate::front_of_house::hosting statement in 
src/lib.rs also hasn’t changed, nor does use have any impact on what files 
are compiled as part of the crate. The mod keyword declares modules, and Rust 
looks in a file with the same name as the module for the code that goes into
that module.

Summary:
Rust lets you split a package into multiple crates and a crate into modules 
so you can refer to items defined in one module from another module. 
You can do this by specifying absolute or relative paths. 
These paths can be brought into scope with a use statement so you can use a 
shorter path for multiple uses of the item in that scope. 
Module code is private by default, but you can make definitions public by 
adding the pub keyword.
*/