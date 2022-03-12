use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Secret number is {}", secret_number);

        println!("Input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("FAILED TO READ LINE");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Needs to be a number try again");
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You got it! well done!");
                break;
            }
            Ordering::Greater => println!("too much"),
            Ordering::Less => println!("too little"),
        }
    }
}
