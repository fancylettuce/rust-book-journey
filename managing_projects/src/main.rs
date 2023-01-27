/*
The module system refers to:
'Packages': A Cargo feature that lets you build, test, and share crates
'Crates': A tree of modules that produces a library or executable
'Modules' and 'use': Let you control the organization, scope, and privacy of paths
'Paths': A way of naming an item, such as a struct, function, or module.

Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers 
at a time.

Crates can contain modules, and the modules may be defined in other files
that get compiled with the crate.

A crate can take one of two forms: binary or library.
Binary crates are programs that you compile to an executable that you can run,
such as a CLI program or a server.

Library crates don't have a 'main' function, and they don't compile to 
an executable. They define functionality intended to be shared with 
multiple projects. Most of the time 'crate' is used to refer to library crates.

The crate root is a source file that the Rust compiler starts from 
and makes up the root module of your crate.

A package is a bundle of one or more crates that provides a set of 
functionality. The package contains a Cargo.toml file that describes how 
to build those crates.

A package can contain as many binary crates as you like, but at most 
only one library crate.
A package must always contain at least one crate, 
whether that's a library or binary crate.

//creating a package
$ cargo new my-project
    Created binary (application) 'my-project' package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs

'cargo new' creates a package.
'ls' lists the files in the package.

If a package contains scr/main.rs and src/lib.rs, 
it has two crates: a binary and a library,both with the same name 
as the package. A package can have multiple binary crates by placing files 
in the src/bin directory: each file will be a separate binary crate.
*/

/*
Defining Modules to Control Scope and Privacy

'paths' allow you to name items.
'use' keyword brings a path into scope.
'pub' keyword makes items public.
'as' keyword is (here)
Glob Operator is

Modules Cheat Sheet

Here we provide a quick reference on how modules, paths, the use keyword, 
and the pub keyword work in the compiler, and how most developers organize 
their code. We’ll be going through examples of each of these rules throughout
this chapter, but this is a great place to refer to as a reminder of 
how modules work.

Start from the crate root: When compiling a crate, the compiler first looks 
in the crate root file (usually src/lib.rs for a library crate or src/main.rs 
for a binary crate) for code to compile.
Declaring modules: In the crate root file, you can declare new modules; 
say, you declare a “garden” module with mod garden;. 
The compiler will look for the module’s code in these places:
Inline, within curly brackets that replace the semicolon following mod garden
In the file src/garden.rs
In the file src/garden/mod.rs
Declaring submodules: In any file other than the crate root, 
you can declare submodules. For example, you might declare mod vegetables; 
in src/garden.rs. The compiler will look for the submodule’s code within 
the directory named for the parent module in these places:
Inline, directly following mod vegetables, within curly brackets instead of 
the semicolon
In the file src/garden/vegetables.rs
In the file src/garden/vegetables/mod.rs
Paths to code in modules: Once a module is part of your crate, 
you can refer to code in that module from anywhere else in that same crate, 
as long as the privacy rules allow, using the path to the code. For example,
an Asparagus type in the garden vegetables module would be found at 
crate::garden::vegetables::Asparagus.
Private vs public: Code within a module is private from its parent modules 
by default. To make a module public, declare it with pub mod instead of mod. 
To make items within a public module public as well, use pub before their 
declarations.
The use keyword: Within a scope, the use keyword creates shortcuts to items 
to reduce repetition of long paths. In any scope that can refer to 
crate::garden::vegetables::Asparagus, you can create a shortcut with 
use crate::garden::vegetables::Asparagus; and from then on you only need 
to write Asparagus to make use of that type in the scope.
Here we create a binary crate named backyard that illustrates these rules. 
The crate’s directory, also named backyard, contains these files and 
directories:

backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
The crate root file in this case is src/main.rs, and it contains:

Filename: src/main.rs

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
The pub mod garden; line tells the compiler to include the code it finds 
in src/garden.rs, which is:

Filename: src/garden.rs

pub mod vegetables;
Here, pub mod vegetables; means the code in src/garden/vegetables.rs is 
included too. That code is:

#[derive(Debug)]
pub struct Asparagus {}
Now let’s get into the details of these rules and demonstrate them in action!
*/
