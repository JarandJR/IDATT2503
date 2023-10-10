use crate::pbkdf2::Pbkdf2;

pub struct Cracker {
    pub hashed_pass: String,
    pub salt: String,
    iterations: u32,
    key_length: usize,
    pbkdf2: Pbkdf2,
    print: bool,
}

impl Cracker {
    pub fn new(hashed_pass: String, salt: String, iterations: u32, key_length: usize, print: bool) -> Self{
        Self {
            hashed_pass,
            salt,
            iterations,
            key_length,
            pbkdf2: Pbkdf2::new(),
            print,
        }
    }

    // The method for cracking the password limited by max password length
    pub fn crack_password(&mut self, max: usize) -> Option<String>{
        println!("cracking ...");
        for i in 1..=max {
            let mut attempt = vec![0; i];
            if let Some(res) =self.brute_force(1, i, &mut attempt) {
                return Some(res);
            }
        }
        None
    }

    // Brute force and guess the password by combining letters in all the possible ways 
    fn brute_force(&mut self, current_len: usize, max_length: usize, attempt: &mut Vec<u8>) -> Option<String> {
        for c in 65..123 {
            attempt[current_len - 1] = c as u8;
            if current_len < max_length {
                if let Some(res) = self.brute_force(current_len + 1, max_length, attempt) {
                    return Some(res);
                }
            } else {
                let at = String::from_utf8_lossy(&attempt).to_string();
                let res = from_bytes_to_hex(self.pbkdf2.derive_key(
                    attempt, 
                    self.salt.as_bytes(), 
                    self.iterations, 
                    self.key_length));

                    if self.print {
                    println!("Atempting: {}", at);
                    //println!("{}\n{}", res[..self.hashed_pass.len()].to_string(), self.hashed_pass);
                }
                if res[..self.hashed_pass.len()] == self.hashed_pass {
                    return Some(at);
                }
            }
        } 
        None
    }

    pub fn tests_pbkdf2(&mut self) {
        let k = self.pbkdf2.derive_key(b"t", b"Saltet til Ola", self.iterations, self.key_length);
        print!("pbkdf2: ");
        for s in k {
            print!("{:x}", s)
        }

        println!("");
    }
}

// Formates bytes into hexadecimal
fn from_bytes_to_hex(bytes: Vec<u8>) -> String {
    let hex_chars: Vec<String> = bytes.iter().map(|b| format!("{:x}", b)).collect();
    hex_chars.join("")
}