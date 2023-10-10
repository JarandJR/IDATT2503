use crate::sha1::Sha1;

pub struct Pbkdf2 {
    sha1: Sha1,
}

impl Pbkdf2 {
    pub fn new() -> Self {
        Self { sha1: Sha1::new() }
    }

    // Calculate the derived key
    pub fn derive_key(&mut self, password: &[u8], salt: &[u8], c: u32, dk_len: usize) -> Vec<u8> {
        let hlen = 20; // SHA-1 output length in bytes
        if dk_len > (u32::MAX - 1) as usize * hlen {
            panic!("Derived key length too long");
        }
        let l = (dk_len + hlen - 1) / hlen;
        let r = dk_len - (l - 1) * hlen;

        let mut dk = vec![];
        for i in 1..=l {
            let t_i = self.f(password, salt, c, i);
            if i == l {
                dk.extend_from_slice(&t_i[..r]);
            } else {
                dk.extend_from_slice(&t_i);
            }
        }
        dk
    }

    // Function f for calculating the xor sum of u1..uc
    fn f(&mut self, password: &[u8], salt: &[u8], c: u32, i: usize) -> Vec<u8> {
        let mut u = Vec::new();
        let mut u_prev = self.prf(password, &[salt, &(i.to_be_bytes())].concat());

        // Calculates u1..uc
        for _ in 1..=c {
            let ui = self.prf(password, &u_prev);
            u_prev = ui.clone();
            u.push(ui);
        }

        // Calculates the xor sum of u1..uc
        let mut xor_sum = vec![0u8; u[0].len()];
        for ui in u.iter() {
            for (res_byte, ui_byte) in xor_sum.iter_mut().zip(ui.iter()) {
                *res_byte ^= ui_byte;
            }
        }
        xor_sum
    }

    // The pseudorandom function
    fn prf(&mut self, key: &[u8], data: &[u8]) -> [u8; 20] {
        self.hmac_sha1(key, data)
        //self.sha1.hash(&[key, data].concat())
    }

    fn hmac_sha1(&mut self, key: &[u8], data: &[u8]) -> [u8; 20] {
        let block_size = 64; // Block size for SHA-1

        // Ensure the key is not longer than the block size
        let mut key_padded = Vec::with_capacity(block_size);
        key_padded.extend_from_slice(key);
        if key.len() > block_size {
            // Hash the key if it's longer than the block size
            key_padded = self.sha1.hash(&key_padded).to_vec();
        }
        if key.len() < block_size {
            // Pad with zeros if shorter
            key_padded.resize(block_size, 0);
        }

        // Prepare the inner and outer padded keys
        let mut o_key_pad = key_padded.clone();
        let mut i_key_pad = key_padded.clone();

        for byte in o_key_pad.iter_mut() {
            *byte ^= 0x5C;
        }
        for byte in i_key_pad.iter_mut() {
            *byte ^= 0x36;
        }

        let mut inner_data = Vec::new();
        inner_data.extend_from_slice(&i_key_pad);
        inner_data.extend_from_slice(data);
        let inner_hash = self.sha1.hash(&inner_data);

        let mut outer_data = Vec::new();
        outer_data.extend_from_slice(&o_key_pad);
        outer_data.extend_from_slice(&inner_hash);
        self.sha1.hash(&outer_data)
    }
}
