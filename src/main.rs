mod utils;
use crate::utils::hashing_string;

fn main() {
    let hash1=hashing_string("hello");
    let hash2=hashing_string("hello");

    println!("this is {}", hash1 == hash2);
}
