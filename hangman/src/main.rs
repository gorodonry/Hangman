use rand::seq::SliceRandom;

mod words;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let word = words::get_words()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

    println!("{}", word);
}
