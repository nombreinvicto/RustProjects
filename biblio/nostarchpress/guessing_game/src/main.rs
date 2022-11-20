use std::io;

// Rng trait must be in scope before it can be used
// this trait has the gen_range() method
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Guess the number! \n Please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);

        //shadowing in same scope but to different type
        // parse can convert to any type, hence you have to
        // explicitly mention type for guess here
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
prod
