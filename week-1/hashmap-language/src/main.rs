use std::collections::HashMap;

/// Return a hashmap with the name of 15 the programming language and its year of creation.
fn languages() -> HashMap<String, u32> {
    let mut languages = HashMap::new();

    languages.insert("Rust".to_string(), 2010);
    languages.insert("Python".to_string(), 1991);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C".to_string(), 1972);
    languages.insert("C++".to_string(), 1983);
    languages.insert("C#".to_string(), 2000);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("Go".to_string(), 2009);
    languages.insert("Swift".to_string(), 2014);
    languages.insert("Kotlin".to_string(), 2011);
    languages.insert("R".to_string(), 1993);
    languages.insert("Ruby".to_string(), 1995);
    languages.insert("Scala".to_string(), 2003);
    languages.insert("Dart".to_string(), 2011);

    languages
}

/// Normalize the years in the given languages map and calculate weights.
fn normalize(languages: &mut HashMap<String, u32>) -> HashMap<String, u32> {
    for year in languages.values_mut() {
       *year = 2024 - *year; 
    }

    let min_year = languages.values().min().unwrap_or(&0);
    let max_year = languages.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, year) in languages.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as u32 + 1;
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    // Print the normalized weights.
    let weights = normalize(&mut languages());
    for (name, weight) in weights {
        println!("{}: {}", name, weight);
    }
}