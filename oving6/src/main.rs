mod sha1;
mod pbkdf2;
mod password_cracker;

use password_cracker::Cracker;
fn main() {
    let mut cracker = Cracker::new(
        String::from("ab29d7b5c589e18b52261ecba1d3a7e7cbf212c6"), 
        String::from("Saltet til Ola"),
        2048,
        32,
        true
    );
    cracker.tests_pbkdf2();
    println!("pbkdf2: d638ddede96a1c1778b6ad1f5d860e93c\n");
    println!("The hashed password: {} \nThe salt used: {}\n", cracker.hashed_pass, cracker.salt);
    let mut response = String::from("Could not crack password ...");
    let res = cracker.crack_password(1);
    if res.is_some() {
        response = res.unwrap();
    }
    println!("\nThe cracked password is:\n{}", response);
}
