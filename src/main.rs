use std::io; //library for user inputs
use rand::Rng; //rand::Rng let's us use the library of random numbers hence: rng (random number generator)
use std::cmp::Ordering;

fn main() {

println!("Guess the number!");

let secret_number = rand::thread_rng().gen_range(1..=100); //thread_rng grabs the rng. gen_range gives the rng a range, we chose 1-100
//println!("The secret number is: {secret_number}"); //We can remove this now since we don't want user to know the #

loop { //creates a loop for the bottom code and closed it off with } at end. Allows user to input multiple times until guess matches with secret #
    println!("Please input your guess:");

    let mut guess = String::new(); //mut means mutable, by default variables are immutable, value can't change. We use mut to make the value change since user input differs.
    
    io::stdin()
        .read_line(&mut guess)//& means its a reference. A refernce gives us a way to let multiple parts of code access one piece of data. Ref (&) are immutable so we have to write &mut to make the ref 'guess' mutable 
        .expect("Failed to read line"); //We write '.expect' as a backup for errors. If errors occur it will read "failed to read line"
    
    let guess: u32 =  match guess.trim().parse() { //trim: eliminates white space, parse: converts a string to another type, hence we convert from string to number
        Ok(num) => num, //We eliminated .expect anmd added "match". In this case: when parse returns a result we get a Ok and Err, we use match to match those outputs. If its a # then we say its ok or pass, if it's not, we say Err to continue program. We use '_' as a catch all value.
        Err(_) => continue,
    };

    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break; //Added a break here to tell program to end after rng and input # match, thus quitting and ending program
            }
        }
    }
}

/* This project was an introduction to Rust concepts such as:
    -let, match functions
    -use of external crates, and add them to the cargo.toml under dependancies *remember to add those dependecies to top of main.rs. (Ex: use rand::Rng;)*

    Next project will be an introduction to variables, data types, and functions and how to use them in Rust
    Self-Reflection: Gosh this was a little hard to memorize and comprehend, but reading the notes in the rust doc was helpful. The concept is easy to memorize once you know how it works. 
        Would say as an intro project it was 8/10, need to reread the project doc to better grasp it, especially the dependacies and external crates.
    To better grasp the feel of Rust formatting and its concepts I'm asking my future self, re write this code in memory. Once you memorize the basic formating it will come naturally.
    */
