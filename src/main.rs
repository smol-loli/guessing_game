use std::cmp::Ordering;
use std::io;
use rand::Rng;

// TODO: put io::stdin (user input) and string to u32 conversion in get_input() function
// TODO: Add colors to text-fields

fn main() {

    let mut _valid_min_max: bool = false;

    while _valid_min_max == false {
        println!("Please select the range for the random number!");

        println!("Minimum:");

        let mut _random_min = String::new();

        io::stdin()
            .read_line(&mut _random_min)
            .expect("Not a number!");

        let _random_min: u32 = match _random_min.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };

        println!("Maximum:");

        let mut _random_max = String::new();

        io::stdin()
            .read_line(&mut _random_max)
            .expect("Not a number!");

        let _random_max: u32 = match _random_max.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };

        if _random_min < _random_max {
            println!("Guess the number!");

            let secret_number = rand::thread_rng().gen_range(_random_min..=_random_max);

            println!("Debug: The number is: {secret_number}");

            _valid_min_max = true;

            guess(secret_number);
        }

        else {
            println!("Please enter a minimum and a maximum that has the value of at least minimum +1");
            continue;
        }
    }

fn guess(secret_number: u32) {

    let mut trys: u32 = 0;

    loop {
        println!("Whats your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        trys += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You only needed {trys} trys!");
                break;
                },
            };
        }
    }
}
