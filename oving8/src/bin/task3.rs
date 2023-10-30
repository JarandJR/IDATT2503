use std::collections::hash_map::HashMap;

const ABC: [char; 32] = [
' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 
'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
'x', 'y', 'z', 'æ', 'ø', 'å', ',', '.',
];

fn main() {
    let abc_map = get_abc_index_map();
    let iv = 13;
    let key = 3;

    println!("aaaaaa -> E: {}", encrypt("aaaaaa", &abc_map, iv, key));
    println!("dette er en test -> E: {}", encrypt("dette er en test", &abc_map, iv, key));

    println!("QVXÆYY HKGDGK,,OQHDNC -> D: {}", decrypt("QVXÆYY HKGDGK,,OQHDNC", &abc_map, iv, key));
}

fn encrypt(m: &str, map: &HashMap<char, usize>, iv: i32, k: i32) -> String {
    let m = m.to_lowercase().chars().collect::<Vec<char>>();
    
    let mut prev_byte = iv;
    let mut res = String::new();
    for &l in &m {
        let i = get_letter_index(l, &map) ^ prev_byte;
        let ch = f(i, k);
        res.push(ABC[ch as usize]);
        prev_byte = ch;
    }
    res.to_uppercase()
}

fn decrypt(m: &str, map:  &HashMap<char, usize>, iv: i32, k: i32) -> String {
    let m = m.to_lowercase().chars().collect::<Vec<char>>();
    
    let mut prev_byte = iv;
    let mut res = String::new();
    for &l in &m {
        let i = get_letter_index(l, &map);
        let ch = f_inv(i, k) ^ prev_byte;
        res.push(ABC[ch as usize]);
        prev_byte = i ;    
    }

    res
}

fn f(x: i32, k: i32) -> i32 {
    (x + k).rem_euclid(ABC.len() as i32)
}

fn f_inv(y: i32, k: i32) -> i32 {
    (y + ABC.len() as i32 - k).rem_euclid(ABC.len() as i32)
}

fn get_abc_index_map() -> HashMap<char, usize> {
    let abc_map: HashMap<char, usize> = {
        let mut map = HashMap::new();
        for (index, &letter) in ABC.iter().enumerate() {
            map.insert(letter, index);
        }
        map
    };
    abc_map
}

fn get_letter_index(l: char, map: &HashMap<char, usize>) -> i32 {
    match  map.get(&l){
        Some(&i) => {
            return i as i32;
        }
        None => {
            panic!("Could not match letter and get index")
        }        
    }
}