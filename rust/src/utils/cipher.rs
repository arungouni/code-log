pub struct CaeserCipher {
    //declare the values required for this configuration
    shift: i32,
    salt: String,
}

impl CaeserCipher {
    pub fn new(shift: i32, salt: String) -> Self {
        //declare the init for this implementation
        CaeserCipher {
            shift,
            salt,
        }
    }

    pub fn encrypt(&self, s: &str) -> String {
        //declare mutable result string and add salt to provided value
        let mut result: String = String::new();
        let val = format!("{}{}", self.salt, s);

        //iterate over each char of provided string
        for ch in val.chars() {
            if ch.is_ascii_alphabetic() {
                // Pick the first letter as per char case
                let first = if ch.is_ascii_lowercase() { 'a' } else { 'A' };

                // Make up the formulae
                // pos + shift ) % 26 **remove the ascii value of A or a from ch.ascii
                let pos = (ch as i32) - (first as i32);
                let shifted = (pos + self.shift).rem_euclid(26);

                // add the ascii value again to the shifted pos
                let result_char = ((first as i32) + shifted) as u8 as char;
                result.push(result_char);
            } else {
                result.push(ch);
            }
        }
        //return the result as String
        return result;
    }
    pub fn decrypt(&self, val: &str) -> String {
        //declare a mutable string to store and return final result
        let mut result: String = String::new();

        //iterate over provided &str
        for ch in val.chars() {
            //check if the item is alphabet
            if ch.is_ascii_alphabetic() {
                // pick the right case of A
                let first = if ch.is_ascii_lowercase() { 'a' } else { 'A' };

                // pos - shift) % 26
                let pos = (ch as i32) - (first as i32);
                let shifted = (pos - self.shift).rem_euclid(26);
                let result_char = ((first as i32) + shifted) as u8 as char;

                //push the decrypted char
                result.push(result_char);
            } else {
                result.push(ch);
            }
        }
        // remove the salt from string
        let final_ = result.replace(&self.salt, "");

        //return the decrypted String
        return final_;
    }
}
