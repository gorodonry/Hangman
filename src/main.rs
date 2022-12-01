use alphabet_macro::alphabet;
use rand::seq::SliceRandom;

mod input;
mod words;

alphabet!(LAT_ALPHABET = "abcdefghijklmnopqrstuvwxyz");

fn main() {
    let word: Vec<char> = words::get_words()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .chars()
        .collect();

    let mut incorrect_guesses: Vec<String> = Vec::new();
    let mut user_progress: Vec<char> = vec!['_'; word.len()];
    let mut guesses_left: i8 = 12;

    let mut user_playing = true;

    // Loop until the users wishes to stop playing.
    while user_playing {
        // Loop until the user guesses the word or runs out of guesses.
        while &user_progress.iter().collect::<String>() != &word.iter().collect::<String>()
            && guesses_left > 0
        {
            println!(
                "With {} guesses remaining you have successfully guessed {}.\n",
                guesses_left,
                &user_progress.iter().collect::<String>()
            );

            // Print out each incorrect letter guessed, separated by commas.
            if incorrect_guesses.len() > 0 {
                println!(
                    "Incorrect letters guessed so far: {}.\n",
                    &incorrect_guesses.join(", ")
                );
            }

            // Obtain a guess from the user, ensure it is valid.
            let mut guess: char = char::default();
            while !LAT_ALPHABET.contains(&guess) {
                let input = input::get_input_with_prompt("Enter your guess: ").to_lowercase();

                println!();

                if incorrect_guesses.iter().any(|x| x == &input) {
                    println!("You have already guessed {input}.\n");

                    continue;
                }

                if input.len() == 1 {
                    guess = input.chars().nth(0).unwrap();
                } else {
                    println!("Please enter a single character.\n");

                    continue;
                }

                if !LAT_ALPHABET.iter().any(|x| x == &guess) {
                    println!("That's not a letter...\n");
                }
            }

            // If the user guessed a letter correctly, update their progress
            if word.contains(&guess) {
                println!("Correct!\n");

                for letter_index in 0..word.len() {
                    if word[letter_index] == guess {
                        user_progress[letter_index] = guess;
                    }
                }
            } else {
                println!("Incorrect!\n");

                incorrect_guesses.push(String::from(guess));

                guesses_left -= 1;
            }
        }

        if &user_progress.iter().collect::<String>() == &word.iter().collect::<String>() {
            println!(
                "Congratulations on guessing my word, {}!\n",
                &word.iter().collect::<String>()
            );
        } else {
            println!(
                "L. You died. The word was {} :)\n",
                &word.iter().collect::<String>()
            );
        }

        user_playing = input::ask_yes_no_question("Would you like to play again?", true);
    }
}
