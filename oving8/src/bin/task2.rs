use oving8::{xor, affine_encrypt, AES};

fn main() {
    let k1_hex = "0123456789ABCDEF0123456789ABCDEF";
    let k2_hex = "1123456789ABCDEF0123456789ABCDEF";
    let x1_hex = "01000000000000000000000000000000";
    let x2_hex = "02000000000000000000000000000000";

    let k1 = hex_to_bytes(k1_hex);
    let k2 = hex_to_bytes(k2_hex);
    let x1 = hex_to_bytes(x1_hex);
    let x2 = hex_to_bytes(x2_hex);

    let result1_otp = xor(&k1, &x1);
    let result2_otp = xor(&k1, &x2);
    let result1_affine = affine_encrypt(&x1, &k1);
    let result2_affine = affine_encrypt(&x2, &k1);

    let mut aes = AES::new(&x1, &k1);
    let result1_aes_one_round = aes.cipher(1);
    let mut aes = AES::new(&x2, &k1);
    let result2_aes_one_round= aes.cipher(1);
    let mut aes = AES::new(&x1, &k1);
    let result1_aes = aes.cipher(10);
    let mut aes = AES::new(&x2, &k1);
    let result2_aes= aes.cipher(10);

    println!("a)\nResult OTP    x1 with k1: {:?}", vec_bytes_to_hex(&result1_otp));
    println!("Result OTP    x2 with k1: {:?}\n", vec_bytes_to_hex(&result2_otp));
    println!("Result Affine x1 with k1: {:?}", vec_bytes_to_hex(&result1_affine));
    println!("Result Affine x2 with k1: {:?}\n", vec_bytes_to_hex(&result2_affine));
    println!("Result 1 AES x1 with k1: {}", bytes_to_hex(&result1_aes_one_round));
    println!("Result 1 AES x2 with k1: {}\n", bytes_to_hex(&result2_aes_one_round));
    println!("Result AES x1 with k1: {}", bytes_to_hex(&result1_aes));
    println!("Result AES x2 with k1: {}", bytes_to_hex(&result2_aes));

    let result2_otp = xor(&k2, &x1);
    let result2_affine = affine_encrypt(&x1, &k2);
    let mut aes = AES::new(&x1, &k2);
    let result2_aes_one_round= aes.cipher(1);
    let mut aes = AES::new(&x1, &k2);
    let result2_aes= aes.cipher(10);
    println!("\nb)\nResult OTP    x1 with k1: {:?}", vec_bytes_to_hex(&result1_otp));
    println!("Result OTP    x1 with k2: {:?}\n", vec_bytes_to_hex(&result2_otp));
    println!("Result Affine x1 with k1: {:?}", vec_bytes_to_hex(&result1_affine));
    println!("Result Affine x1 with k2: {:?}\n", vec_bytes_to_hex(&result2_affine));
    println!("Result 1 AES x1 with k1: {}", bytes_to_hex(&result1_aes_one_round));
    println!("Result 1 AES x1 with k2: {}\n", bytes_to_hex(&result2_aes_one_round));
    println!("Result AES x1 with k1: {}", bytes_to_hex(&result1_aes));
    println!("Result AES x1 with k2: {}", bytes_to_hex(&result2_aes));
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .expect("Failed to convert hex to u8")
        })
        .collect()
}

fn vec_bytes_to_hex(bytes: &Vec<u8>) -> String {
    bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

fn bytes_to_hex(bytes: &[[u8; 4]; 4]) -> String {
    let mut res = String::from("\n");
    for row in bytes {
        res.push_str(
            &row.iter()
                    .map(|b| format!("{:02x}, ", b))
                    .collect::<Vec<String>>()
                    .join("")[..]
        );
        res.push_str("\n");
    }
    res
}