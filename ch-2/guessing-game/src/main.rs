use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("========= Guess the number! =========");
    
    // generate a random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!();
        println!("Please input your guess: ");

        let mut guess = String::new(); // returns a new instance of a String

        // read a line from the standard input to the string
        // if the read fails, the program will crash and display the message
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin");

        // convert guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare guess to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("========== Finished! ==========");

}
