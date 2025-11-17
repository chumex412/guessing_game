use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Print guessing number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guessing = String::new();

        io::stdin()
            .read_line(&mut guessing)
            .expect("Failed to read line");

        let guessing: u32 = match guessing.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("You guessed: {guessing}");
        match guessing.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
