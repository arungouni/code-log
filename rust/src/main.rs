mod utils;
use std::env;
use std::fs;

fn main() {
    //get command args
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Invalid Arguments");
        return;
    }
    let filename = args[1].clone();
    let val = fs::read_to_string(filename).expect("Unable to read file");
    //parse args to strings
    let shift: i32 = args[3].parse().unwrap();
    let salt: &str = &args[2];
    //declare cipher and config to perform encrypt and decrypt
    let cipher = utils::cipher::CaeserCipher::new(shift, salt.to_string());
    let encrypted = cipher.encrypt(&val);
    //debug the implementation
    println!("Encrypted Value: {}", encrypted);
    println!("Original/ Decrypted Value: {}", cipher.decrypt(&cipher.encrypt(&val)));
}
