fn main() {
    // prompt the user for their name
    println!("Hello, what is your name?");
    let mut name: String = String::new();
    match std::io::stdin().read_line(&mut name) {
        Ok(n) => {
            println!("Nice to meet you '{}'", name);
            println!("Bytes read {}", n);
        }
        Err(error) => println!("error: {}", error),
    }
}
