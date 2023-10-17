pub fn run(){
    println!("Task 2\n---------------------\na)");
    let table = get_mod_multiplication_table(12);
    print_2d_table(&table);

    println!("\n---------------------\nb)");
    let table = get_multiplicative_inverse_entries(table);
    println!("Multiplicative inverse for n=12: {:?}", table);

    println!("\n---------------------\nc)");
    let table = get_multiplicative_inverse_entries(get_mod_multiplication_table(11));
    println!("Multiplicative inverse for n=11: {:?}", table);
    
    println!("\n---------------------\nd)");
    let table = brute_force_multiplicative_inverse(11, 29);
    println!("Multiplicative inverse for 11 mod 29: {:?}", table);

    println!("\n---------------------\ne)");
    println!("If \'a\' has a multiplicative inverse modulo \'n\' it means that it has no common factors other than 1");

    println!("\n---------------------\n");
}

fn get_mod_multiplication_table(n: i32) -> Vec<Vec<i32>> {
    let mut table: Vec<Vec<i32>> = Vec::new();
    for i in 1..n {
        let mut row: Vec<i32> = Vec::new();
        for j in 1..n {
            row.push((i*j)%n);
        }
        table.push(row);
    }
    table
}

fn get_multiplicative_inverse_entries(t: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut table: Vec<(usize, usize)> = Vec::new();
    for i in 0..t.len() {
        for j in 0..t.len() {
            if *t.get(i).unwrap().get(j).unwrap() == 1 {
                table.push((i+1, j+1));
            }
        }
    }
    table
}

fn brute_force_multiplicative_inverse(a: i32, n: i32) -> Vec<i32> {
    let mut table: Vec<i32> = Vec::new();
    for b in 0..=n {
        if (a*b)%n == 1 {
            table.push(b);
        }
    }
    table
}

fn print_2d_table(table: &Vec<Vec<i32>>) {
    println!("Table:\n");
    for i in 0..table.len() {
        for j in 0..table.get(i).unwrap().len() {
            println!("({} * {}) % {} = {}", i+1, j+1, table.len()+1, table.get(i).unwrap().get(j).unwrap());
        }
        println!()
    }
}