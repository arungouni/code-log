mod utils;
use std::env;
use std::fs;

fn main() {
    //get command args
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Invalid Arguments");
        return;
    }
    let operation = &args[1];
    if operation == "encrypt" {
        let filename = args[2].clone();
        let val = fs::read_to_string(filename).expect("Unable to read file");
        //parse args to strings
        let shift: i32 = args[4].parse().unwrap();
        let salt: &str = &args[3];
        //declare cipher and config to perform encrypt and decrypt
        let cipher = utils::cipher::CaeserCipher::new(shift, salt.to_string());
        let encrypted = cipher.encrypt(&val);
        match std::fs::write("files/encrypt.txt", &encrypted) {
            Ok(_) => println!("File written successfully!"),
            Err(e) => panic!("Failed to write: {e}"),
        }
    } else if operation == "decrypt" {
        let filename = args[2].clone();
        let val = fs::read_to_string(filename).expect("Unable to read file");
        //parse args to strings
        let shift: i32 = args[4].parse().unwrap();
        let salt: &str = &args[3];
        //declare cipher and config to perform encrypt and decrypt
        let cipher = utils::cipher::CaeserCipher::new(shift, salt.to_string());
        let decrypted = cipher.decrypt(&val);
        match std::fs::write("files/decrypt.txt", &decrypted) {
            Ok(_) => println!("File written successfully!"),
            Err(e) => panic!("Failed to write: {e}"),
        }
    } else {
        eprintln!("Invalid Operation");
        return;
    }
}
