fn pig_latin(word: String) -> String {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let vowel_suffix = "hay";
    let consonant_suffix = "ay";
    let mut latinized: String = String::from("");

    if word.starts_with(vowels) {
        latinized.push_str(&word);
        latinized.push('-');
        latinized.push_str(vowel_suffix);
    } else {
        let remainder: String = word.chars().skip(1).collect();
        latinized.push_str(&remainder);
        latinized.push('-');
        if let Some(c) = word.chars().next() {
            latinized.push(c);
        }
        latinized.push_str(consonant_suffix);
    }

    latinized
}

fn main() {
    assert_eq!("irst-fay", pig_latin(String::from("first")));
    assert_eq!("apple-hay", pig_latin(String::from("apple")));
}
