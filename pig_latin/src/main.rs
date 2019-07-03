fn pig_latinise_word(word: &String) -> String {
    let mut charlist = word.chars();
    let initial_char = match charlist.next() {
        Some(c) => c,
        None => return String::from("Empty String"),
    };
    match initial_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => format!("{}-hay", word),
        _ => format!("{}-{}ay", charlist.as_str(), initial_char),
    }
}

fn pig_latinise_sentence(sentence: &String) -> String {
    let mut out_word_list = Vec::new();
    for word in sentence.split_whitespace() {
        out_word_list.push(pig_latinise_word(&String::from(word)));
    };
    out_word_list.join(" ")
}

fn main() {
    let input = String::from("Come on, let me see you pig-latinise this!");
    let output = if input.is_ascii() {
        pig_latinise_sentence(&input)
    } else {
        String::from("Sorry, I can only handle ascii strings for now")
    };
    println!("{}\n\tbecomes\n{}", input, output);
}
// It folds punctuation into the neighbouring word, but apart from that seems to work.