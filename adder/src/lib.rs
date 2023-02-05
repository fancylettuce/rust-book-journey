//pub fn add(left: usize, right: usize) -> usize {
  //  left + right
//}

// the test module and function generated automatically by cargo new
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/

//showing exploration instead of it_works
//adding a second test that will fail because we call the panic! macro
/*
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
*/

/*
#[derive(Debug)]
struct Rectangle {  // using the rectangle struct and its can_hold method from chapter 5
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
*/

// a test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
/*
#[cfg(test)]
mod tests {
    use super::*;

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        panic! (larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
*/

/*
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

*/

/*
struct candle {
    if let then light kek Lok 'Tar, Ogar!
}
*/

/*
pub fn greeting(name: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
*/

/*
pub struct Guess {
   pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            ); 
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
    let _ = &Guess::new(200);
    }
}
*/

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

*/
/*
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
*/

/*
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {

}
    
    /*
        #[test]
        fn add_two_and_two() {
            assert_eq!(4, add_two(2));
        }
    
        #[test]
        fn add_three_and_two() {
            assert_eq!(5, add_two(3));
        }
    
        #[test]
        fn one_hundred() {
            assert_eq!(102, add_two(100));
        }
    }
    */

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {

}
*/

#[cfg(test)]
mod tests {
    #[test]() {
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
}


