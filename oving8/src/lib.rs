pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter())
        .map(|(byte_a, byte_b)| byte_a ^ byte_b)
        .collect()
}

pub fn affine_encrypt(m: &Vec<u8>, k: &Vec<u8>) -> Vec<u8>{
    let modulo = 256;
    let a = k[0];
    let b = k[1];

    if !gcd(a as u64, modulo as u64) == 1 {
        panic!("a should be chosen to be relatively prime to m")
    }

    let mut c = Vec::new();
    for &byte in m.iter() {
        c.push((((a * byte + b)as i32).rem_euclid(modulo)) as u8);
    }
    c
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a.rem_euclid(b);
        a = temp;
    }
    a
}

pub struct AES {
    m: [[u8;4];4],
    k: [[u8;4];4],
    sub_table: [[u8;16];16],
    mix_matrix: [[u8;4];4],
    rcon_table: [[u8; 10];4],
}

impl AES {
    pub fn new(m: &Vec<u8>, k: &Vec<u8>) -> AES{
        AES {
            m: modify_message(m),
            k: modify_key(k),
            sub_table: get_sub_table(),
            mix_matrix: get_mix_columns_matrix(),
            rcon_table: get_rcon_table(),
        }
    }

    pub fn cipher(&mut self, rounds: usize) -> [[u8; 4]; 4] {
        self.add_round_key();
        for i in 0..rounds {
            if i < 9 {
                self.cipher_round(i);
            } else {
                self.cipher_last_round(i);
            }
        }
        self.m
    }

    fn cipher_round(&mut self, round: usize) {
        self.key_schedule(round);
        self.sub_bytes();
        self.shift_rows();
        self.mix_columns();
        self.add_round_key();
    }

    fn cipher_last_round(&mut self, round: usize) {
        self.key_schedule(round);
        self.sub_bytes();
        self.shift_rows();
        self.add_round_key();
    }

    fn key_schedule(&mut self, rcon: usize) {
        let mut res_key = [[0; 4]; 4];
        let mut temp_col = [0; 4];

        for i in 0..4 {
            temp_col[i] = self.k[i][3];
        }
        let tmp = temp_col[0];
        temp_col[0] = temp_col[1];
        temp_col[1] = temp_col[2];
        temp_col[2] = temp_col[3];
        temp_col[3] = tmp;

        temp_col = self.sub_column(&temp_col);

        for i in 0..4 {
            res_key[i][0] = self.k[i][0] ^ temp_col[i] ^ self.rcon_table[i][rcon];
        }
    
        for j in 1..4 {
            for i in 0..4 {
                res_key[i][j] = res_key[i][j - 1] ^ self.k[i][j];
            }
        }
  
        self.k = res_key;
    }

    fn sub_column(&mut self, col: &[u8;4]) -> [u8;4]{
        let mut res = [0;4];
        for i in 0..4 {
            res[i] = self.sub_table[col[i] as usize >> 4][col[i] as usize & 0xF];
        }
        res
    }

    fn sub_bytes(&mut self) {
        for i in 0..4 {
            let mut res = [0;4];
            for j in 0..4 {
                res[j] = self.sub_table[self.m[i][j] as usize >> 4][self.m[i][j] as usize & 0xF];
            }
            self.m[i] = res;
        }
    }

    fn shift_rows(&mut self) {
        let mut res = [[0; 4]; 4];
        // First row
        res[0] = self.m[0];
        // Second row: Shift left by 1 position
        res[1][0] = self.m[1][1];
        res[1][1] = self.m[1][2];
        res[1][2] = self.m[1][3];
        res[1][3] = self.m[1][0];
        // Third row: Shift left by 2 positions
        res[2][0] = self.m[2][2];
        res[2][1] = self.m[2][3];
        res[2][2] = self.m[2][0];
        res[2][3] = self.m[2][1];
        // Fourth row: Shift left by 3 positions
        res[3][0] = self.m[3][3];
        res[3][1] = self.m[3][0];
        res[3][2] = self.m[3][1];
        res[3][3] = self.m[3][2];

        self.m = res;
    }

    fn mix_columns(&mut self) {
        for col in 0..4 {
            let mut temp = [0; 4];
            for row in 0..4 {
                for i in 0..4 {
                    temp[row] ^= galois_mult(self.mix_matrix[row][i], self.m[i][col]);
                }
            }
            for row in 0..4 {
                self.m[row][col] = temp[row];
            }
        }
    }

    fn add_round_key(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = self.m[i][j] ^ self.k[i][j];
            }
        }   
    }
}

fn galois_mult(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0;
    let mut hi_bit_set;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        hi_bit_set = a & 0x80 != 0;
        a <<= 1;
        if hi_bit_set {
            a ^= 0x1B;
        }
        b >>= 1;
    }
    p
}

fn modify_message(m: &Vec<u8>) -> [[u8;4];4] {
    if m.len() != 16 {
        panic!("Message size mismatch")
    }
    let mut mes = [[0;4];4];
    let mut counter = 0;
    for i in 0..4 {
        for j in 0..4 {
            mes[j][i] = *m.get(counter).unwrap();
            counter += 1;
        }
    }
    mes
}

fn modify_key(k: &Vec<u8>) -> [[u8;4];4] {
    if k.len() != 16 {
        panic!("Key size mismatch")
    }
    let mut key = [[0;4];4];
    let mut counter = 0;
    for i in 0..4 {
        for j in 0..4 {
            key[j][i] = *k.get(counter).unwrap();
            counter += 1;
        }
    }
    key
}

fn get_mix_columns_matrix() -> [[u8; 4]; 4] {
    [
        [0x02, 0x03, 0x01, 0x01],
        [0x01, 0x02, 0x03, 0x01],
        [0x01, 0x01, 0x02, 0x03],
        [0x03, 0x01, 0x01, 0x02],
    ]
}

fn get_rcon_table() -> [[u8; 10]; 4] {
    [
        [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ]
}

fn get_sub_table() -> [[u8;16];16] {
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
    ]
}