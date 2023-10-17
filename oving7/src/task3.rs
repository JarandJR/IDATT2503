pub fn run(){
    println!("Task 3\n---------------------\na)");
    let alp = get_alphabet_array();
    for i in 0..=28 {
        let f_res = f(i as i32);
        println!("{} = f({}) = {} = {}",alp.get(i).unwrap(), i, f_res, alp.get(f_res as usize).unwrap());
    }
    println!("\n---------------------\nb)");
    println!("
    The function f is a permutation of Z_29, meaning it's a one-to-one and onto function.

    Injectivity: If f(x) = f(y), then x must be equal to y. This function preserves distinctness.
    
    Surjectivity: For every element z in Z_29, there exists an element x such that f(x) = z. 
    This function covers all elements in Z_29.
    
    Therefore, f is a permutation of Z_29, meaning it bijectively maps the elements of Z_29 to themselves.");

    println!("\n---------------------\nc)");
    /*
    y = (11*X -5) % 29
    x = (11*y -5) % 29
    x + 5 = 11*y % 29
    8*(x + 5) % 29=y
    f^-1 = 8*(x + 5) % 29
            40 % 29 = 11
    f^-1 = 8*x + 11 % 29
    */
    let a = 8;
    let b = 11;
    for i in 0..=28 {
        let f_inv = f_inverse(i as i32, a, b);
        println!("{} = f^-1({}) = {} = {}",alp.get(i).unwrap(), i, f_inv, alp.get(f_inv as usize).unwrap());
    }

    println!("\n---------------------\nd)");
    println!("encrypt: alice -> {}", encrypt("alice"));

    println!("\n---------------------\ne)");
    println!("decrypt: SIØPBE -> {}", decrypt("SIØPBE"));

    println!("\n---------------------\n");
}

fn get_alphabet_array() -> Vec<char> {
    "abcdefghijklmnopqrstuvwxyzæøå".chars().collect()
}

fn get_alphabet_hashmap() -> std::collections::HashMap<char, usize> {
    let mut map = std::collections::HashMap::new();
    for (i, &l) in get_alphabet_array().iter().enumerate() {
        map.insert(l, i);
    }
    map
}

fn f(x: i32) -> i32 {
    (11 * x - 5).rem_euclid(29)
}

fn f_inverse(y: i32, a: i32, b: i32) -> i32 {
    (a * y + b).rem_euclid(29)
}

fn encrypt(message: &str) -> String {
    let map = get_alphabet_hashmap();
    let alp = get_alphabet_array();
    let message = message.chars().collect::<Vec<char>>();

    let mut res = String::new();
    for &l in &message {
        match map.get(&l) {
            Some(&i) => {
                let f_res = f(i as i32) as usize;
                res.push(alp[f_res]);
            }
            None => {
                panic!("Could not match letter and get index")
            }
        }
    }
    res.to_uppercase()
}

fn decrypt(message: &str) -> String {
    let map = get_alphabet_hashmap();
    let alp = get_alphabet_array();
    let message = message.to_lowercase().chars().collect::<Vec<char>>();

    let mut res = String::new();
    for &l in &message {
        match map.get(&l) {
            Some(&i) => {
                let f_res = f_inverse(i as i32, 8, 11) as usize;
                res.push(alp[f_res]);
            }
            None => {
                panic!("Could not match letter and get index")
            }
        }
    }
    res
}