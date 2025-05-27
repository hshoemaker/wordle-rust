use std::io;

fn main() {
    println!("Welcome to Wordle!");
    println!("\t-: Letter isn't in the word\n\t+: Letter is in the word but in the wrong place\n\t*: Letter is in the word and in the right place");
    let word = "guess";
    let mut number_of_attempts_left = 6;

    println!("The wordle is: {word}");

    loop {
        let mut guess = String::new();
        println!("Input your guess. You have {number_of_attempts_left} attempts left.");
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        if guess.trim().len() != 5 {
            println!("Your guess isn't 5 characters long. Please guess a valid, 5 letter word.");
            continue;
        }
        
        number_of_attempts_left = number_of_attempts_left - 1;

        if guess.trim() == word {
            println!("*****");
            println!("You win! Your score is {}", 6 - number_of_attempts_left);
            break;
        } else {
            let mut chars = &guess.trim().char_indices();
            let mut wordChars = &word.char_indices();
            
            println!("--*-+");
        }

        if number_of_attempts_left <= 0 {
            break;
        }
    }
}

// Function to Fetch Dictionary of 5 letter words

// Function to determine the word for the game
// Use this api: https://random-word-api.herokuapp.com/word?length=5

// Function to check the input vs the word