mod utils;
use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
    eprintln!("Usage: {} <filepath> <shift>", args[0]);
    return;
}
    let filepath: &str = &args[1];
    let shift: i32 = args[2].parse().expect("Shift must be a integer");
    let content = fs::read_to_string(filepath).expect("Failed to read file");
    println!("Original Value: {}", content);
    let encrypted = utils::cipher::encrypt(&content, shift);
    let decrypted = utils::cipher::decrypt(&encrypted, shift);
    println!("Encrypted value: {}", encrypted);
    println!("Decrypted value: {}", decrypted);
    let _ = fs::write("file_decrypted.txt", decrypted).expect("Failed to decrypt content");


}

pub fn compress(s: &str) -> String{
let mut prev: Option<char> = None;
let mut count = 0;
let mut result = String::new();

for ch in s.chars(){
    if prev == None{
        prev = Some(ch);
            count = 1;
        } else if prev == Some(ch){
            count += 1;
        } else {
            result.push(ch);
            result.push_str(&count.to_string());
            prev = Some(ch);
            count = 1;
            }
}
result
}
