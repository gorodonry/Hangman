use alphabet_macro::alphabet;
use rand::seq::SliceRandom;

mod gallows;
mod input;
mod words;

alphabet!(LAT_ALPHABET = "abcdefghijklmnopqrstuvwxyz");

fn main() {
    let mut user_playing = true;

    // Loop until the users wishes to stop playing.
    while user_playing {
        let word: Vec<char> = words::get_words()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .chars()
            .collect();

        let mut incorrect_guesses: Vec<String> = Vec::new();
        let mut correct_guesses: Vec<String> = Vec::new();
        let mut user_progress: Vec<char> = vec!['_'; word.len()];
        let mut guesses_left: i8 = 12;

        // Loop until the user guesses the word or runs out of guesses.
        while &user_progress.iter().collect::<String>() != &word.iter().collect::<String>()
            && guesses_left > 0
        {
            println!("{}\n", gallows::get_gallow_ascii(guesses_left as usize));

            println!(
                "With {} guesses remaining you have successfully guessed {}.\n",
                guesses_left,
                &user_progress.iter().collect::<String>()
            );

            // Print out each incorrect letter guessed, separated by commas.
            if incorrect_guesses.len() > 0 {
                println!(
                    "Incorrect guesses so far: {}.\n",
                    &incorrect_guesses.join(", ")
                );
            }

            // Obtain a guess from the user, ensure it is valid.
            let mut char_guess = char::default();
            let mut word_guess = String::new();
            'input_loop: while !LAT_ALPHABET.contains(&char_guess) && word_guess == String::new() {
                let input = input::get_input_with_prompt("Enter your guess: ").to_lowercase();

                println!();

                // Check that the user hasn't already guessed the input.
                if incorrect_guesses.iter().any(|x| x == &input)
                    || correct_guesses.iter().any(|x| x == &input)
                {
                    println!("You have already guessed {input}.\n");

                    continue;
                }

                // Check the guess for validity.
                if input.len() == 1 {
                    char_guess = input.chars().nth(0).unwrap();
                } else if input.len() == word.len() {
                    // Check that each letter of the input is in the alphabet.
                    for letter in (&input).chars() {
                        if !LAT_ALPHABET.iter().any(|x| x == &letter) {
                            println!("Not all of those are letters...\n");

                            continue 'input_loop;
                        }
                    }

                    word_guess = input;

                    continue;
                } else {
                    println!("Invalid number of characters!\n");

                    continue;
                }

                if !LAT_ALPHABET.iter().any(|x| x == &char_guess) {
                    println!("That's not a letter...\n");
                }
            }

            // Process the guess.
            if word_guess != String::new() {
                // If the user tried to guess the entire word at once, check their guess.
                if &word.iter().collect::<String>() == &word_guess {
                    user_progress = word.clone();
                } else {
                    println!("No, that's not it!\n");

                    incorrect_guesses.push(word_guess);
                }
            } else if word.contains(&char_guess) {
                // If they guessed a single letter, check the letter.
                println!("Correct!\n");

                for letter_index in 0..word.len() {
                    if word[letter_index] == char_guess {
                        user_progress[letter_index] = char_guess;
                    }
                }

                correct_guesses.push(String::from(char_guess));
            } else {
                println!("Incorrect!\n");

                incorrect_guesses.push(String::from(char_guess));

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
                "{}\n\nL. You died. The word was {} :)\n",
                gallows::get_gallow_ascii(0),
                &word.iter().collect::<String>()
            );
        }

        user_playing = input::ask_yes_no_question("Would you like to play again?", true);
    }
}
