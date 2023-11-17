fn main() {
    let d = 3;
    let p = 1283;
    let q = 2027;

    println!("a)\nwith p: {}\nshe should choose a big prime number for q\nso that |(q - p)| is as large of a number as possible", p);
    println!("The q she chose was: {}", q);
    println!("\nFor a weak choice of q,\nFermats factorization is a good method of attack to factorize n");
    
    println!("\nb)");
    let (e, _k)  = extended_euclidian_algorithm(-d, (p-1)*(q-1));
    println!("e: {}", e);

    println!("\nc)");
    let message = 111;
    let n = p * q;
    println!("Encrypted message: {}", mod_exp(message, e.into(), n.into()));
}

fn extended_euclidian_algorithm(a: i32, b: i32) -> (i32, i32) {
    let (mut a, mut b) = (a, b);
    let (mut x, mut y, mut u, mut v) = (0, 1, 1, 0);
    while a != 0 {
        let (q, r) = (b / a, b.rem_euclid(a));
        let (m, n) = (x - u * q, y - v * q);
        (b, a, x, y, u, v) = (a, r, u, v, m, n);
    }
    return (x, y);
}

fn mod_exp(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let mut c = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp % 2 == 1 {
            c = (c * base).rem_euclid(modulus);
        }
        exp >>= 1;
        base = (base * base).rem_euclid(modulus);
    }

    c
}
