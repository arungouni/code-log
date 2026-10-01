// The encrypt function below
pub fn encrypt(val: &str, shift: i32) -> String {
    let mut result = String::new();

    for ch in val.chars() {
        if ch.is_ascii_alphabetic() {
            let first = if ch.is_ascii_lowercase() { 'a' } else { 'A' };

            let pos = ch as i32 - first as i32;

            let shifted = (pos + shift).rem_euclid(26);

            let result_char = (first as i32 + shifted) as u8 as char;

            result.push(result_char);
        } else {
            result.push(ch);
        }
    }

    result
}


// The decrypt function below
pub fn decrypt(val: &str, shift: i32) -> String {
    let mut result = String::new();

    for ch in val.chars() {
        if ch.is_ascii_alphabetic() {
            let first = if ch.is_ascii_lowercase() { 'a' } else { 'A' };

            let pos = ch as i32 - first as i32;

            let shifted = (pos - shift).rem_euclid(26);

            let result_char = (first as i32 + shifted) as u8 as char;

            result.push(result_char);
        } else {
            result.push(ch);
        }
    }

    result
}

