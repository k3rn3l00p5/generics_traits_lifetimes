// Generics help handle duplication of concepts
// Generics are abstract stand-ins for concrete types or other properties
// functions can take parameters of some generic type instead of a concrete type
// Option<T>, Vec<T>, HashMap<K, V> are Result<T, E> are all generics
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

fn main() {
    let number_list = vec![1, 2, 3, 5, 4];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number in array is {}", largest);

    // instead of duplicating code we can call a function that take a list and pass the list into the function
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let number_list2 = vec![1, 2, 3, 6, 5, 8, 4, 7];
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("Largest number in array is {}", largest_i32(&number_list));
    println!("Largest number in array is {}", largest_i32(&number_list2));
    println!("Largest number in array is {}", largest_char(&char_list));

    let integer = Point { x: 5, y: 1.0 };
    let float = Point { x: 5.2, y: 2 };

    // trait usage
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// to eliminate code duplication we create an abstraction by defining a function that operates on any list of integers given to it in a parameter
fn largest_i32(list: &[i32]) -> i32 {
    // list parameter represents a reference to any concrete slice of i32 values
    // references are used because it won't create new data when passed into the function (direct access to the variables passed into the function)
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// code duplication again (instead with the parameters taken into the function)
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// to parameterize the we first name the type parameter usually T by convention (short for type and CamelCase)
// to define a generic largest function place type name declarations inside angle brackets (<>) between name of function and parameter list
fn largest_any<T: PartialOrd + Copy>(list: &[T]) -> T {
    // bound generic to PartialOrd trait because some data types can't be compared using >
    // bound generic to Copy because some data types don't implement the Copy trait (anything that isn't stored on the stack)
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we can use generics to create definitions for items like function signatures or structs
// generics are placed in the signature of the function where normally data types of the parameter and return value go

// in struct definitions
struct Point<T, U> {
    // it's also possible to use multiple generic types for different type associations between variants
    x: T,
    y: U,
}

// what the option enum looks like
enum Option<T> {
    // when Some(3) is called it will create a variant of Some with the value of 3 inside it (rust infers the data type is i32 on instantiation)
    Some(T), // any value type associated and the value stored in a Some variant is the T
    None,    // no associated value type just a name
}

// what the result enum looks like
// let test: Result<i32, i32> = Result::Ok(2);
enum Result<T, E> {
    // result enum is a generic over T and E
    // Two variants where ok holds T generic and err holds E generic
    Ok(T), // a passed result will call Result::Ok(type);
    Err(E),
}

// in method definitions
// allows a generic type on a impl method call for the point struct with a generic
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// implementing a function on certain instances of a struct
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// a trait tells rust compiler about functionality a particular type has and can share with other types
// traits are used to define shared behavior in an abstract way
// trait bounds to specify that a generic can be any type that has certain behavior
// a type's behavior consists of the methods we can call on that type
// different types can share the same behaviors if we can call the same method on all of those types
// trait definitions allow us to group method signatures together to define a set of behaviors
pub trait Summary {
    // trait is declared with a name
    // inside this block we declare the method signatures that describe the behaviors of the types that implement this trait

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // traits can have default behavior (which we can override method behavior if we want)
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// we can implement the trait on the types we want by using impl for
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
// with this commented out the default behavior for the summarize method will just print Read more... for any instance of the NewsArticle struct

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits have to be in the same scope as what they are being implemented on
// traits also have to be public for another crate to implement it outside of it's scope
// can't implement external traits on external types (one has to be local to the crate)
// this restriction is part of coherence (orphan rule)
// parent type is not present
// without the restrictions two crates could implement the same trait for the same type and rust won't know which to use

// traits can also be used as function parameters
// this is impl Trait syntax but there is a longer syntax called trait bound syntax
// pub fn notify(item: impl Summary) {
pub fn notify<T: Summary>(item: T, item2: T) {
    // pub fn notify<T: Summary + Display>(item: T) {
    // trait bound syntax is better for more complex instances where we can have two parameters that implement Summary1
    // it's also possible to bind multiple traits using + syntax
    // Type is now bound to Summary trait type and item is allowed to be of any data type
    // instead of a concrete type for the item parameter we specify the impl keyword and the trait name
    // this accepts any type that implements the specified trait (so far 2 structs)
    // in the body of notify we can call any methods on item that come from the Summary trait (like summarize or summarize_author)
    // we call notify and pass in any instance of NewsArticle or Tweet
    // code that calls the function with any other type won't compile
    println!("Breaking news! {}", item.summarize());
}

// using too many trait bounds has its downsides
// each generic has its own trait bounds so functions with multiple generic type parameters can contain lots of trait bound information between the function's name and its parameter list
// instead of fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Summary + Clone,
    U: Clone + Summary,
{
    10
}

// returning a value of some type that implements a trait using impl trait syntax
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// conditionally implemented methods

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// implements on all types
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// will only implement on types that implement Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// it's also possible to implement a trait for any type that implements another trait
// implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are used through the standard library
//impl<T: Display> ToString for T {
//    / for example the standard library implements the ToString trait on any type that implements the Display trait
//}

// every reference in rust has a lifetime
// a lifetime is the scope for which that reference is valid
// most of the time lifetimes are implicit and inferred
fn reference_lifetimes() {
    let r;

    {
        let x = 5;
        r = &x;
    } // because r stores a reference to x and x is dropped here there is an issue

    println!("r: {}", r);
}
