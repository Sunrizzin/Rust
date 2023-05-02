use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number equal {}", secret_number);
    loop {
    println!("Please, enter your number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error for read string!");

    let guess: u32 = guess.trim().parse().expect("Please, enter number!");
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