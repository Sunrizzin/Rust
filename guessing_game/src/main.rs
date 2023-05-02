use std::io;

fn main() {
    println!("Guess number!");
    println!("Please, enter your number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error for read string!");

    println!("You enter: {}", guess)
}