use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruit = ["Orange", "Apple", "Banana", "Pear", "Grape"];

    // Scramble (shuffle) the vector
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

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
