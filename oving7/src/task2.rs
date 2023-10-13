pub fn task2(){
    println!("Task 2\n---------------------\na)");
    let table_12 = get_mod_multiplication_table(12);
    print_2d_table(table_12);
    println!("---------------------\n");
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

fn print_2d_table(table: Vec<Vec<i32>>) {
    println!("Table:\n");
    for i in 0..table.len() {
        for j in 0..table.len() {
            println!("({} * {}) % {} = {}", i+1, j+1, table.len()+1, table.get(i).unwrap().get(j).unwrap());
        }
        println!()
    }
}