fn main() {
    println!("2)\na)");
    let k = 0b1001;
    let m = 0b0110;
    println!("hmac(m) = {:04b}", hmac(m, &k));

    println!("\nb)");
    let recived_m = 0b0111;
    let recived_hmac = 0b0100;
    if hmac(recived_m, &k) == recived_hmac {
        println!("HMAC is valid");
    } else {
        println!("HMAC is invalid");
    }
}

fn hmac(m: u8, k: &u8) -> u8 {
    let ipad = 0b0011;
    let opad = 0b0101;
    h((k^opad) + h((k^ipad) + m))
}

fn h(x: u8) -> u8 {
    let hash = (x as u16).pow(2) as u8;
    (hash >> 2) & 0b1111
}