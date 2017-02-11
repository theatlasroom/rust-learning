fn main() {
    println!("Starting");
    let mut x = ("Knight", "A1", "B3");
    let y = ("Bishop", "A2", "B3");
    println!("Move X {:?}", x);
    println!("Move Y {:?}", y);
    x = y;
    println!("Move X {:?}", x);
    println!("Move Y {:?}", y);
}
