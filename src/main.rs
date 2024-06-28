use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let torrent = fs::read_to_string(file_path).expect("Should be able to read file.");

    println!("{}", torrent);
}
