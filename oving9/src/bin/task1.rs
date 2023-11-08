fn main() {
    println!("a)\n");
    let k = [0b1000, 0b0011, 0b1111].to_vec();
    let periods = get_periods(&k, recursive_sequence_a);
    for i in 0..periods.len() {
        println!("K: {:04b}: {}", k[i], periods[i]);
    }

    println!("\nb)\n");
    let periods = get_periods(&k, recursive_sequence_b);
    for i in 0..periods.len() {
        println!("K: {:04b}: {}", k[i], periods[i]);
    }
}

fn get_periods(k: &Vec<u8>, recursive_sequence: fn(& u8)->u8) -> Vec<u8>{
    let mut res = Vec::new();
    k.iter().for_each(|initial| {
        let mut state = *initial;
        let mut period = 0;
        loop {
            state = recursive_sequence(&state);
            period += 1;

            if state == *initial {
                break;
            }
        }
        res.push(period);
    });
    res
}

fn recursive_sequence_a(x: &u8) -> u8 {
    let mut bits = Vec::with_capacity(4);
    for i in 0..4 {
        bits.push((x >> i) & 1);
    }
    bits.reverse();
    let new_bit = bits.iter().fold(0, |a, b| a + b).rem_euclid(2);
    (new_bit << 3) | (x >> 1)
}

fn recursive_sequence_b(x: &u8) -> u8 {
    let mut bits = Vec::with_capacity(2);
    for i in [0,3].iter() {
        bits.push((x >> i) & 1);
    }
    bits.reverse();
    let new_bit = bits.iter().fold(0, |a, b| a + b).rem_euclid(2);
    (new_bit << 3) | (x >> 1)
}
