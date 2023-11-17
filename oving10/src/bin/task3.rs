fn main() {
    println!("a)");
    let n = 1829;
    let b = 5;
    let p = pollard_p(n, b);
    println!("p = {}\n", p);

    println!("b)");
    let n = 18779;
    println!("The smallest B for a successful pollard p-1 attack on n = 18779 is given by the smallest of the last Q values for p-1 and q-1. p-1: Q= (2,11), q-1: Q=(2,3,5,7). So the smallest Q-value of the last is 7. Testing the pollard p-1 attack with B = 7 can verify its success.");
    println!("p: {}", pollard_p(n, 1000));
}

fn pollard_p(n: i64, b: i64) -> i64 {
    let mut a = 2;
    for i in 1..=b {
        a = mod_exp(a, i, n);
        let f = gcd(a - 1, n);
        if f > 1 {
            println!("B: {}", i);
            return f;
        }
    }
    return -1;
}

fn mod_exp(mut a: i64, mut b: i64, n: i64) -> i64 {
    let mut d = 1;
    while b > 0 {
        if b % 2 == 1 {
            d = (d * a) % n;
        }
        a = (a * a) % n;
        b /= 2;
    }
    d
}


fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
