use std::{collections::HashMap};

fn logic(numbers: Vec<i32>) -> HashMap<i32, u32> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frecuency = frequencies.entry(number).or_insert(0);
        *frecuency += 1;
    }

    frequencies
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 7, 7, 5, 6, 1, 7, 1, 8, 2, 2,2,2,9, 10];
    let result = logic(numbers);

    println!("The frequency of each number in the vector is: {:?}", result);
}
