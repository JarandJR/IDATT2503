fn main() {
    println!("4)\na)");
    let nonce = 0b01100101;
    a(&nonce);
    println!("\nb)\nPeriode: {}", find_period(&nonce));

    println!("\nc)\n
    The computation of the keystream can be parallelized, 
    because each block of the keystream is generated independently of the others.
    In CTR mode, a nonce and a counter are used to generate the input for each block cipher operation.
    The output of the block cipher operation is then XORed with the plaintext to produce the ciphertext.
    Because the nonce and counter can be computed in advance, and because the block 
    cipher operation on each block is independent, 
    these operations can be performed in parallel.");

    println!("\nd)\n 
    If you have a known plaintext-ciphertext pair,
    you can recover the corresponding part of the key-stream.
    This is because the encryption operation is simply XORing the plaintext with the key-stream
    to produce the ciphertext. 
    Therefore, if you XOR the ciphertext with the plaintext, you get back the key-stream.

    The formula: ciphertext XOR plaintext = keystream
    
    Once you have the key-stream, 
    you can use it to decrypt other parts of the ciphertext that were encrypted with the
    same part of the key-stream.
    
    However, knowing the key-stream doesn't necessarily give you information about the key used
    to generate the key-stream,
    especially if a secure block cipher is used.
    The relationship between the key and the key-stream should be complex enough that you can't
    derive the key from the key-stream.");
}

fn a(nonce: &u16) {
    for counter in 0..4 {
        let combined = (nonce + counter).rem_euclid(256) as usize;
        let byte = SUB_TABLE[combined >> 4][combined & 0xF];
        println!("Byte for counter {}: {:08b}", counter, byte);
    }
}

fn find_period(nonce: &u16) -> usize {
    let mut key_stream = Vec::new();
    let mut counter = 0;

    loop {
        let combined = (nonce + counter).rem_euclid(256) as usize;
        let byte = SUB_TABLE[combined >> 4][combined & 0xF];

        if key_stream.contains(&byte) {
            return key_stream.len();
        }

        key_stream.push(byte);
        counter += 1;
    }
}

const SUB_TABLE: [[u16;16];16] = 
    [
        [0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,],
        [0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,],
        [0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,],
        [0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,],
        [0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,],
        [0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,],
        [0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,],
        [0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,],
        [0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,],
        [0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,],
        [0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,],
        [0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,],
        [0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,],
        [0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,],
        [0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,],
        [0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,],
    ];