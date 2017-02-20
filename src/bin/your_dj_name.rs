// return your dj name
// take random strings and mash them togethor

fn main() {
    let names = ["James", "Joan", "Willow", "Darren", "Phil", "Ashley", "Kate", "Carla"];
    let genres = ["Blues", "Rock", "Pop", "Jazz", "Funk", "Soul", "Rap"];
    let dj_tuple: &(String, String, String) = dj_name_generator(&names, &genres);
    let dj_name = tuple_to_string(&dj_tuple);
    println!("Your DJ name is: {:?}", dj_name);
}

fn dj_name_generator(names: &[str], genres: &[str]) -> (str, str, str) {
    let name = names[0];
    let genre = genres[0];
    let res = ("DJ".to_string(), name.to_string(), genre.to_string());
    &res;
}

fn tuple_to_string(tup: (&str, &str, &str)) -> str {
    let postfix: String = ", the great";
    let result: &str = tup.0 + tup.1 + tup.2 + &postfix;
    &result;
}
