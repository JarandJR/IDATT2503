fn main() {
    let k = 3;
    let x = 0b1101111110100001;
    let res = cbc_mac(x, k, e);
    println!("x: {:016b}, CBC-MAC: {:08b}", x, res);

    let x = 0b0010110000011111;
    let res = cbc_mac(x, k, e_inv);
    println!("x': {:016b}, CBC-MAC: {:08b}", x, res);
}

fn cbc_mac(m: u32, k: u32, e: fn(u32, u32)-> u32) -> u32 {
    let mut y = 0b0000;
    for i in [4, 8, 12, 16].iter() {
        let x = ((m >> 16 - i) & 0b1111) ^ y;
        y = e(x, k);
    }
    y
}

fn e(x: u32, k: u32) -> u32 {
    (x + k).rem_euclid(256)
}

fn e_inv(y: u32, k: u32) -> u32 {
    (y + 256 - k).rem_euclid(256)
}