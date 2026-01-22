use sha2::{Sha256,Digest};


pub fn hashing_string(input:&str)->Vec<u8> {

    let mut hasher=Sha256::new();
    hasher.update(input.as_bytes());
    let result= hasher.finalize();

    println!("{:x}", result);
    return result.to_vec();
}
