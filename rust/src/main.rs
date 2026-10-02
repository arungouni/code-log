mod utils;
use std::env;

fn main() {
    //get command args
    let args: Vec<String> = env::args().collect();

    //parse args to strings
    let shift: i32 = args[3].parse().unwrap();
    let val: String = args[1].clone();
    let salt: &str = &args[2];
    //declare cipher and config to perform encrypt and decrypt
    let cipher = utils::cipher::CaeserCipher::new(shift, salt.to_string());

    //debug the implementation
    println!("Encrypted Value: {}", cipher.encrypt(&val));
    println!("Original/ Decrypted Value: {}", cipher.decrypt(&cipher.encrypt(&val)));
}
