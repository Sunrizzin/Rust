use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, enter your number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error for read string!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You enter: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is so small!"),
            Ordering::Greater => println!("Number is so bug!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}