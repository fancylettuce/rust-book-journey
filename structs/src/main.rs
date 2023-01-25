/*
fn main() {
    println!("Hello, world!");
}

*/


/* Rust is CSS with commas --

*/

/*
struct dot = object;
struct color = u32, u32, u32, u32;
fn main() {
    let color = (042, 069, 069, 042,);
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square need moar dots.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

*/

//Refactoring with Tuples
/*fn main() {
    let bouth_groups = (30, 50);

    println!(
        "The area of bouth groups on the tail is {} squared need ahmoar dots on the otherside!",
        area(bouth_groups)
    );
}

fn area(dime_engines: (u32, u32)) -> u32 {
    dime_engines.0 * dime_engines.1
}

*/

//Refactoring with Structs: Adding More Meaning
/*struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let bouth_groups = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels on every time.",
        area(&bouth_groups)
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

*/

//Adding Useful Funuctionality with Derived Traits
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale =2;
    let bouth_groups = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&bouth_groups);
}
*/

//Method Syntaax
//Defining Methods
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(bouth_groups) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let bouth_groups = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Don't stand near {} whjelp groups.",
        bouth_groups.area()
    );
}
*/

//width
/*impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let bouth_groups = Rectangle {
        width: 30,
        height: 50,
    };

    if bouth_groups.width() {
        println!("odd groups go left even groups nonzero width; it is {}", bouth_groups.width);
    }
}
*/

//Where's the -> Operator?
/*
In C and C++, two different operators are used for calling methods: 
you use . if you’re calling a method on the object directly and -> 
if you’re calling the method on a pointer to the object and need to 
dereference the pointer first. In other words, if object is a pointer, 
object->something() is similar to (*object).something().

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a 
feature called automatic referencing and dereferencing. Calling methods is 
one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with object.something(), 
Rust automatically adds in &, &mut, or * so object matches the signature of 
the method. In other words, the following are the same:

pl.distance(&p2);
(&p1).distance(&p2);

The first one looks much cleaner. 
This automatic referencing behavior works because methods have a 
clear receiver—the type of self. Given the receiver and name of a method, 
Rust can figure out definitively whether the method is reading (&self), 
mutating (&mut self), or consuming (self). The fact that Rust makes 
borrowing implicit for method receivers is a big part of making ownership 
ergonomic in practice.

*/

//Methods with More Parameters
/*fn main() {
    let bouth_groups = Rectangle {
        width: 30,
        height 50,
    };
    let odd_groups = Rectangle {
        width: 10,
        height: 40,
    };
    let even_groups = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can bouth_groups hold even_groups? {}", bouth_groups.can_hold(&even_groups));
    println!(" can bouth_groups hold odd_groups? {}", bouth_groups.ca_hold(&odd_groups));
}
*/

/*
Can bouth_groups hold even_grous? true
can bouth_groups hold odd_groups false

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

*/

//Associated Functiona
/*impl Rectangle {
    fn square(size: u32) -> self {
        self {
            width: size,
            height: size,
        }
    }
}
*/

//Mjultiple imple Blocks
/*impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
*/