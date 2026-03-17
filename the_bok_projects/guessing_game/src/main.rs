use std::io;

use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("input:");

        io::stdin().read_line(&mut guess).expect("Failed!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", guess);
                println!("Please type a number!");
                continue;
            }
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
}
