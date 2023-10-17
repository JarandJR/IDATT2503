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
    println!("Task 5\n---------------------\na)");
    let message = "Snart helg".replace(" ", "").to_lowercase();
    let key = "torsk";
    println!("\'Snart helg\' encrypted with \'torsk\' -> {}", vegenere_encrypt(&message, key));

    println!("\n---------------------\nb)");
    let message = "QZQOBVCAFFKSDC".to_lowercase();
    let key = "brus";
    println!("\'QZQOBVCAFFKSDC\' decrypted with \'brus\' -> {}", vegenere_decrypt(&message, key));

    println!("\n---------------------\nc)");
    println!("\nIf the keyword has a length of 15 it results in 29^15 different keys.
    \nThis big word makes it quite secure against a brute force attack");

    println!("\n---------------------\n");
}

fn vegenere_decrypt(message: &String, key: &str) -> String{
    let message = message.chars().collect::<Vec<char>>();
    let key = pad_key(key, &message);

    let mut res = String::new();
    for i in 0..message.len() {
        let k = get_letter_index(key[i]);
        let l = get_letter_index(message[i]);
        let f_res = f_inverse(l as i32, k);
        res.push(ALPHABET[f_res]);      
    }
    res
}

fn vegenere_encrypt(message: &String, key: &str) -> String{
    let message = message.chars().collect::<Vec<char>>();
    let key = pad_key(key, &message);

    let mut res = String::new();
    for i in 0..message.len() {
        let k = get_letter_index(key[i]);
        let l = get_letter_index(message[i]);
        let f_res = f(l as i32, k);
        res.push(ALPHABET[f_res]);
    }
    res.to_uppercase()
}

fn get_letter_index(l: char) -> i32 {
    match  ALPHABET_MAP.get(&l){
        Some(&j) => {
            return j as i32;
        }
        None => {
            panic!("Could not match letter and get index")
        }        
    }
}

fn pad_key(key: &str, message: &Vec<char>) -> Vec<char>{
    let mut key = key.chars().collect::<Vec<char>>();
    let mut counter = 0;
    let original_len = key.len();
    while key.len() != message.len() {
        key.push(key[counter]);
        counter += 1;
        if counter == original_len {
            counter = 0;
        }
    }
    key
}

fn f(y: i32, k: i32) -> usize{
    (y + k).rem_euclid(ALPHABET.len() as i32) as usize
}

fn f_inverse(y: i32, k: i32) -> usize{
    (y - k).rem_euclid(ALPHABET.len() as i32) as usize
}