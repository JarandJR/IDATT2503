pub fn task1() {
    println!("Task 1\n---------------------\na)");
    let a = 17;
    let b = 27;
    let n = 5;
    println!("a = {}, b= {}, n = {}", a, b, n);

    let remainder_a = a % n;
    let remainder_b = b % n;
    if remainder_a == remainder_b {
        println!("a ≡ b (mod n)");
        println!("because: a%n = {} = b%n = {}", remainder_a, remainder_b);
    } else {
        println!("a ≡/ b (mod n)");
        println!("because: a%n = {} != b%n = {}", remainder_a, remainder_b);
    }

    println!("\n---------------------\nb)");
    let res = -99 % 1001;
    println!("-99 mod 1001 = {}", res);

    println!("\n---------------------\nc)");
    let res = (232 + (22 * 77) - (18 as i32).pow(3)) % 8;
    println!("232 + 22 * 77 - 18^3 % 8 = {}", res);
    let res = (232 + 22*77 - (18 as i32).pow(3)) % 8;
    println!("232 + {} - {} % 8 = {}",22*77, (18 as i32).pow(3), res);
    let res = -3906 % 8;
    println!("{} % 8 = {}",232+1694-5832, res);
    let res = -2 % 8;
    println!("-2 % 8 = {}", res);

    println!("\n---------------------\nd)");
    let a = 55;
    let b = 77;
    let n = 12;
    println!("a = {}, b= {}, n = {}", a, b, n);

    let remainder_a = a % n;
    let remainder_b = b % n;
    if remainder_a == remainder_b {
        println!("a ≡ b (mod n)");
        println!("because: a%n = {} = b%n = {}", remainder_a, remainder_b);
    } else {
        println!("a ≡/ b (mod n)");
        println!("because: a%n = {} != b%n = {}", remainder_a, remainder_b);
    }
    println!("---------------------\n");
}