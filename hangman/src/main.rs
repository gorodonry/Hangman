use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

mod words;

const ALPHABET: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

fn main() {
    let word: &str = words::get_words().choose(&mut rand::thread_rng()).unwrap();

    let mut incorrect_letters_guessed: Vec<char> = Vec::new();
    let mut user_progress: Vec<u8> = vec!['_' as u8; word.len()];
    let mut guesses_left: i8 = 12;

    println!("{word}");

    // Loop until the user guesses the word.
    while String::from_utf8_lossy(&user_progress) != word && guesses_left > 0 {
        println!(
            "With {} guesses remaining you have successfully guessed {}.\n",
            guesses_left,
            String::from_utf8_lossy(&user_progress)
        );

        // Print out each incorrect letter guessed, separated by commas.

        // Obtain a guess from the user, ensure it is within the alphabet.
        let mut guess = String::new();
        while !ALPHABET.iter().any(|x| x == &guess.as_str()) {
            guess = get_input_with_prompt("Enter your guess: ").to_lowercase();

            if !ALPHABET.iter().any(|x| x == &guess.as_str()) {
                println!("That's not a letter...");
            }

            println!();
        }

        guesses_left -= 1;
    }
}

fn get_input_with_prompt(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{prompt}");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}
