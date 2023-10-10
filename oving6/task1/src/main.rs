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
        false,
        true
    );
    cracker.tests_pbkdf2();
    println!("pbkdf2: ec74e7e43ea237aa006c532928ba7478dbedcfc5\n");

    println!("The hashed password: {} \nThe salt used: {}\n", cracker.hashed_pass, cracker.salt);
    let mut response = String::from("Could not crack password ...");
    let res = cracker.crack_password(5);
    if res.is_some() {
        response = res.unwrap();
    }
    println!("\nThe cracked password is:\n{}", response);
}
