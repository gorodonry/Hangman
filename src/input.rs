use std::io;
use std::io::Write;

/// Uses `io::stdin` to get input from the user preceded by a prompt.
/// Returns the input provided by the user with as a trimmed `String`.
///
/// # Arguments
///
/// * `prompt` - A string slice given to the user as an input prompt.
///
/// # Examples
///
/// ```
/// let name = get_input_with_prompt("Enter your name: ");
/// ```
pub fn get_input_with_prompt(prompt: &str) -> String {
    let mut buffer = String::new();

    print!("{prompt}");

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

/// Asks the user a question with a binary answer.
/// Returns the input provided by the user as a Boolean,
/// where `"yes"` becomes `true`, and vice versa.
///
/// # Arguments
///
/// * `prompt` - A string slice given to the user as an input prompt.
/// * `default` - The default answer (`true` or `false`; i.e. `"yes"` or `"no"`).
///
/// # Examples
///
/// ```
/// let answer = ask_yes_no_question("Do you like rust?", true);
/// ```
///
/// Output:<br>
/// `Do you like rust? [Y/n]`<br><br>
/// Default is yes (selected if the user hits \[return\]), options are \[y\] and \[n\].
pub fn ask_yes_no_question(prompt: &str, default: bool) -> bool {
    let mut input: String = String::new();

    while !["y", "n"].iter().any(|x| x == &input) {
        // If the default answer is yes.
        if default {
            input = get_input_with_prompt(&format!("{} [Y/n] ", &prompt)).to_lowercase();

            input = if input == String::new() {
                String::from("y")
            } else {
                input
            };
        } else {
            input = get_input_with_prompt(&format!("{} [y/N] ", &prompt)).to_lowercase();

            input = if input == String::new() {
                String::from("n")
            } else {
                input
            };
        }

        println!();

        if !["y", "n"].iter().any(|x| x == &input) {
            println!("Please enter either y or n.\n");
        }
    }

    input == String::from("y")
}
