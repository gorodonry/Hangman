use rand::seq::SliceRandom;

static WORDS: [&str; 114] = [
    "syzygy", "kelp", "rhythym", "quagga", "sky", "why", "cranium",
    "barf", "pun", "query", "square", "wheel", "fly", "seep", "if",
    "squelch", "snowflake", "lynx", "kilt", "zilch", "shyly", "we",
    "flip", "brick", "marsh", "bog", "thy", "thine", "folly", "of",
    "epee", "spot", "foil", "clip", "truck", "hack", "weka", "the",
    "scarab", "bulge", "pi", "fetid", "obtuse", "strobe", "fester",
    "pester", "a", "scourge", "drama", "frigate", "obsolete", "ox",
    "feral", "fiery", "strewn", "weak", "sulfate", "lengthy", "by",
    "strength", "strengthened", "straight", "scarf", "length", "i",
    "fence", "nobility", "alfalfa", "dignity", "banana", "stencil",
    "able", "gram", "sparrow", "tree", "plant", "shrub", "truffle",
    "branch", "ranch", "lief", "limb", "shrug", "climb", "compass",
    "squirrel", "quill", "view", "shyly", "albeit", "spade", "bug",
    "gnat", "sheer", "club", "oak", "sync", "scry", "spry", "gley",
    "ability", "gley", "mollusc", "snob", "help", "enigma", "yarn",
    "fidget", "caveat", "moron", "gnaw", "struck", "jury", "shack"
    ];

fn main() {
    let word: String = WORDS.choose(&mut rand::thread_rng()).unwrap().to_string();

    println!("{}", word);
}
