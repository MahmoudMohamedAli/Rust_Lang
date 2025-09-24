use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "The secret number should be between 1 and 100, got {value}."
            );
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn compare(&self, secrect_num: &i32) -> bool
    {
        match self.value.cmp(&secrect_num)
        {
            Ordering::Greater => {
                println!("the nummber is too big");
                false
            }
            Ordering::Less => {
                println!("the number is too small");
        false}
            Ordering::Equal => {
                println!("u win");
            true}
        }
    }
}

fn main() {
    'main_loop: loop
    {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nGuess the number!");
    let secret_number = rand::thread_rng().gen_range(1..5);
    // println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            // Result is an enum that has the variants Ok or Err
            Ok(num) => num,
            Err(_) => continue, // ask for another guess
        };
        // error handling

        let myguess = Guess::new(guess);
        println!("You guessed: {}", guess);
        let mut answer = String::new() ;
        let res = myguess.compare(&secret_number);
        if res {
            println!("do u want to play again please enter 'yes' or 'y'!");
            io::stdin().read_line(&mut answer).expect("failed to read line");
            let answer = answer.trim().to_lowercase();
            match answer.as_str() {
                "yes" | "y" => continue 'main_loop,
                _ => break 'main_loop,
            }
            
        }
    }
}
}