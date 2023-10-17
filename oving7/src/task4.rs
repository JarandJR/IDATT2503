use std::collections::HashMap;
use lazy_static::lazy_static;

const ALPHABET: [char; 29] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 
    'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', 'æ', 'ø', 'å'
];
lazy_static! {
    static ref ALPHABET_MAP: HashMap<char, usize> = {
        let mut map = HashMap::new();
        for (index, &letter) in ALPHABET.iter().enumerate() {
            map.insert(letter, index);
        }
        map
    };
}

pub fn run(){
    println!("Task 4\n---------------------\n");
    let encrypted = "YÆVFB VBVFR ÅVBV".replace(" ", "").to_lowercase();
    for k in 0..ALPHABET.len() {
        println!("k: {} -> decrypt: {}", k, decrypt(&encrypted, k))
    }
    println!("\nk = 17 -> hjernen er alene = {}", decrypt(&encrypted, 17));
    println!("\n---------------------\n");
}

fn decrypt(message: &String, k: usize) -> String{
    let message = message.chars().collect::<Vec<char>>();

    let mut res = String::new();
    for &l in &message {
        match  ALPHABET_MAP.get(&l){
            Some(&i) => {
                let f_res = f_inverse(i as i32, k as i32);
                res.push(ALPHABET[f_res]);
            }
            None => {
                panic!("Could not match letter and get index")
            }        
        }
    }
    res
}

fn f_inverse(y: i32, k: i32) -> usize{
    (y - k).rem_euclid(ALPHABET.len() as i32) as usize
}
