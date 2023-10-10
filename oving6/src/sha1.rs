pub struct Sha1 {
    h0: u32,
    h1: u32,
    h2: u32,
    h3: u32,
    h4: u32,
    cons_k: [u32; 4],
}

impl Sha1 {
    pub fn new() -> Self {
        Self { 
            h0: 0x67452301,
            h1: 0xEFCDAB89,
            h2: 0x98BADCFE,
            h3: 0x10325476,
            h4: 0xC3D2E1F0,
            cons_k: [
                0x5A827999,
                0x6ED9EBA1,
                0x8F1BBCDC,
                0xCA62C1D6,
            ],
         }
    }

    // Hash the input
    pub fn hash(&mut self, message: &[u8]) -> [u8; 20] {
        let padded_message = self.pad_message(message);
        // Padded message 512-bit into 4 chunks of 64-bit 
        let chunks = padded_message.chunks_exact(64);
        for chunk in chunks {
            let w = self.break_into_words(chunk);
            self.process_words_into_hash(w);
        }
        let hh = self.combine_into_hash();
        self.reset();
        hh
    }
    
    // Padding the message for Sha-1
    fn pad_message(&self, message: &[u8]) -> Vec<u8> {
        // Original message length
        let ml = (message.len() as u64) * 8;
        let mut padded_message = message.to_vec();
        padded_message.push(0x80); // Adds 1 bit
        // Reservs the last 64-bits for the original message length
        while (padded_message.len() * 8) % 512 != 448 {
            padded_message.push(0x00);
        }
        padded_message.extend(&ml.to_be_bytes());
        padded_message
    }
    
    // Break each chunk into 16 32-bit big-endian words
    // Then Extend the 16 words into 80 32-bit words
    fn break_into_words(&self, chunk: &[u8]) -> [u32; 80] {
        let mut w = [0u32; 80];
        
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4], 
                chunk[i * 4 + 1], 
                chunk[i * 4 + 2], 
                chunk[i * 4 + 3],
                ]);
            }
            
            for i in 16..80 {
                w[i] = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
                w[i] = w[i].rotate_left(1);
            }
            w
        }
        
    // Process each 80-word block into the hash
    fn process_words_into_hash(&mut self, w: [u32; 80]) {
        let mut a = self.h0;
        let mut b = self.h1;
        let mut c = self.h2;
        let mut d = self.h3;
        let mut e = self.h4;

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), self.cons_k[0]),
                20..=39 => (b ^ c ^ d, self.cons_k[1]),
                40..=59 => ((b & c) | (b & d) | (c & d), self.cons_k[2]),
                60..=79 => (b ^ c ^ d, self.cons_k[3]),
                _ => unreachable!(),
            };
        
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(w[i]).wrapping_add(k);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        
        self.h0 = self.h0.wrapping_add(a);
        self.h1 = self.h1.wrapping_add(b);
        self.h2 = self.h2.wrapping_add(c);
        self.h3 = self.h3.wrapping_add(d);
        self.h4 = self.h4.wrapping_add(e);
    }
        
    // Combine the 5 32-bit hash values into 160-bit final hash
    fn combine_into_hash(&self) ->  [u8; 20] {
        let mut h = [0u8; 20];
        h[0..4].copy_from_slice(&self.h0.to_be_bytes());
        h[4..8].copy_from_slice(&self.h1.to_be_bytes());
        h[8..12].copy_from_slice(&self.h2.to_be_bytes());
        h[12..16].copy_from_slice(&self.h3.to_be_bytes());
        h[16..20].copy_from_slice(&self.h4.to_be_bytes());
        h
    }

    // Reset the hash values so it can be used again
    fn reset(&mut self) {
        self.h0 = 0x67452301;
        self.h1 = 0xEFCDAB89;
        self.h2 = 0x98BADCFE;
        self.h3 = 0x10325476;
        self.h4 = 0xC3D2E1F0;
    }
}