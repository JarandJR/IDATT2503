use ring::pbkdf2;
use crate::pbkdf2::Pbkdf2;

pub struct Cracker {
    pub hashed_pass: String,
    pub salt: String,
    iterations: u32,
    key_length: usize,
    pbkdf2: Pbkdf2,
    print: bool,
    lib: bool,
}

impl Cracker {
    pub fn new(hashed_pass: String, salt: String, iterations: u32, key_length: usize, print: bool,lib: bool) -> Self{
        Self {
            hashed_pass,
            salt,
            iterations,
            key_length,
            pbkdf2: Pbkdf2::new(),
            print,
            lib,
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
                let res =
                if self.lib { self.hash_using_lib(&attempt)}
                else {
                    hex::encode(self.pbkdf2.derive_key(
                        attempt,
                        self.salt.as_bytes(),
                        self.iterations,
                        self.key_length
                    ))
                };
                if self.print {println!("Atempting: {}", String::from_utf8_lossy(&attempt).to_string());}
                if res[..self.hashed_pass.len()] == self.hashed_pass {
                    return Some(String::from_utf8_lossy(&attempt).to_string());
                }
            }
        }
        None
    }

    // Hashes the attempt using library
    fn hash_using_lib(&mut self, attempt: &[u8]) -> String {
        let mut hash = [0u8; 20];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA1,
            std::num::NonZeroU32::new(self.iterations).unwrap(),
            self.salt.as_bytes(),
            attempt,
            &mut hash,
        );
        hex::encode(hash)
    }

    // Test-function for testing the pbkd2 implementation as it does not work correctly atm
    pub fn tests_pbkdf2(&mut self) {
        let k = self.pbkdf2.derive_key(b"Hei", self.salt.as_bytes(), self.iterations, self.key_length);
        let k_slice = &k[..20];

        let k_hex = hex::encode(k_slice);

        print!("pbkdf2: ");
        print!("{}", k_hex);

        println!("");
    }
}