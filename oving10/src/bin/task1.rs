fn main() {
    let mut n = 275621053;
    println!("n = {}", n);
    let (p, q) = fermats_factorization_method(&mut n);
    println!("p = {}, q = {}", p, q);
    println!("Prime p: {}", is_prime(p as i64));
    println!("Prime q: {}", is_prime(q as i64));
}

fn fermats_factorization_method(n: &i64) -> (f64, f64) {
    let n = n.clone() as f64;
    let mut a = (n as f64).sqrt().abs().ceil();
    loop {
        let b = ((a.powf(2.0) - n)).abs().sqrt();
        if b.fract() == 0.0 {
            return (a - b, a + b)
        } else if a  > b {
            a += 1.0;
        } else {
            panic!("n is prime");
        }
    }
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
