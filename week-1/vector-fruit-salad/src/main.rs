//! Shuffle a vector
//! # What is a Vector and how is different from a arrays?
//! In Rust, a vector is a dynamic array that can grow or shrink in size. It is a collection type that is part of 
//! the standard library and is represented by the Vec type.
//! 
//! The main difference between arrays and vectors in Rust is that arrays have a fixed size determined at compile time,
//! while vectors can chane in size at runtime. Vectors also have additionla methods for dynamic operations such as adding,
//! removing, and modifying elements, while arrays do not have these methods.
//!
//! # What is the use of the SliceRangom trait from the rand crate in the program?
//! The `SliceRandom` trait from the `rand` crate is used in the program to shuffle the elements of the vector. It provides the
//! `shuffle` whitch shuffles the elements of the slice in a random order. In this case, it's used to shuffle the elments of the 
//! `fruit` array before printing out the "Fruit salad".
//!
//! # Why is enumerate method used while printing the fruits? What functionality does it provide?
//! The `enumerate` method is used to iterate over the elements of the `fruit` array and provide both the index and the value
//! of each element in the iteration.
//! This functionality is useful when you want to access both the index and the value of each element in a collection, for example,
//! when you want to print out the index along with the value in a formatted output.

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruit = ["Orange", "Apple", "Banana", "Pear", "Grape"];

    // Scramble (shuffle) the vector
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Challenge(2): The SliceRandom trait provides a method choose(&self, rng: &R) -> Option<&T>. 
    // Can you use this to select a random fruit from the salad?
    let random_fruit = fruit.choose(&mut rng);
    println!("Random fruit: {}", random_fruit.unwrap());

    // Print out the fruit salad
    println!("Fruit salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
