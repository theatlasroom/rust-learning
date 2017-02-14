// basic file IO operations
use std::fs;
use std::env;


// fn write_file(&file, data){
// }
//
// fn read_file(&file){
// }
//
// fn open_file(){
// }

fn main(){

    // We assume that we are in a valid directory.
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());

    let f = "./data/text-file.txt";
    let metadata = fs::metadata(f).unwrap();
    println!("Is {:?} a valid file: {:?}", f, metadata.is_file());

    // open the file for reading
    let mut handle = match fs::File::open(f){
        Some(handle) => println!("File\n{:?}", handle),
        Err => println!("Error\n{:?}", handle),
    };
}
