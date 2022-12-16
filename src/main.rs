use std::cmp::Ordering;
use std::io;
use rand::Rng;

// TODO: Add colors to text-fields (find out how)

fn main() {

    let mut _valid_min_max: bool = false;

    while _valid_min_max == false {
        println!("Please select the range for the random number!");

        println!("Minimum:");

        let _random_min = get_usr_input();

        println!("Maximum:");

        let _random_max = get_usr_input();

        if _random_min < _random_max {
            println!("Guess the number!");

            let secret_number = rand::thread_rng().gen_range(_random_min..=_random_max);

            println!("Debug(main): The secret number is: {secret_number}");

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

        let guess = get_usr_input();

        println!("You guessed: {guess}");

        trys += 1;

        println!("Debug(guess): The secret number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You only needed {trys} trys!");
                break;
                },
            };
        }
    }



    fn get_usr_input() -> u32 {

        let _usr_input: u32 = loop {

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    print!("Error: Not a valid input!");
                    continue;
                },
            };
            return input
        };
    }
}
