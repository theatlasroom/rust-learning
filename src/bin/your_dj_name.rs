// return your dj name
// take random strings and mash them togethor
extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let names = &["James", "Joan", "Willow", "Darren", "Phil", "Ashley", "Kate", "Carla"];
    let genres = &["Blues", "Rock", "Pop", "Jazz", "Funk", "Soul", "Rap"];
    let dj_tuple: (String, String, String) = dj_name_generator(names, genres);
    let dj_name = tuple_to_string(dj_tuple);
    println!("Your DJ name is: {:?}", dj_name);
}

fn dj_name_generator(names: &[&str], genres: &[&str]) -> (String, String, String) {
    let name_index = random(0, names.len());
    let genre_index = random(0, genres.len());
    let name = names[name_index];
    let genre = genres[genre_index];
    let res = ("DJ".to_string(), name.to_string(), genre.to_string());
    res
}

fn tuple_to_string(tup: (String, String, String)) -> String {
    let postfix: String = ", the great".to_string();
    let result: String = tup.0 + " " + &tup.1 + " " + &tup.2 + &postfix;
    result
}

// TODO: this should be in a seperate module for utils
// returns a random number within a range specified
fn random(min: usize, max: usize) -> usize {
    let between = Range::new(min, max);
    let mut rng = rand::thread_rng();
    let number = between.ind_sample(&mut rng);
    println!("random {:?}", number);
    number as usize
}
