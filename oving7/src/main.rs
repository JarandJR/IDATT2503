mod task1;
mod task2;
mod task3;
mod task4;
mod task5;

use std::{thread::sleep, time::Duration};

fn main() {
    println!("Oving 7\n---------------------");
    task1::run();
    sleep(Duration::from_secs(4));
    task2::run();
    sleep(Duration::from_secs(4));
    task3::run();
    sleep(Duration::from_secs(4));
    task4::run();
    sleep(Duration::from_secs(4));
    task5::run();
}