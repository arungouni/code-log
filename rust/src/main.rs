mod utils;

fn main() {
    //declare cipher and config to perform encrypt and decrypt
    let cipher = utils::cipher::CaeserCipher::new(3, "hello".to_string());

    //debug the implementation
    println!("Encrypted Value: {}", cipher.encrypt("arungouni!"));
    println!("Original/ Decrypted Value: {}", cipher.decrypt(&cipher.encrypt("arungouni!")));
}
