fn main() {
    let s = "convert strings to pig latin the first consonant of each word is moved to the end of the word and ay is added so first becomes irst-fay words that start with a vowel have hay added to the end instead apple becomes apple-hay keep in mind the details about UTF-8 encoding";
    const CONSONANTS: [char; 42] = [
        'B', 'C', 'D', 'F', 'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'S', 'T', 'V', 'X', 'Z', 'H',
        'R', 'W', 'Y', 'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v',
        'x', 'z', 'h', 'r', 'w', 'y',
    ];

    let mut pig_lating_s = "".to_string();
    for i in s.split_whitespace() {
        let first_letter = i.chars().nth(0).unwrap();
        if CONSONANTS.contains(&first_letter) {
            let mut root = i.chars();
            root.next();
            let mut root = root.as_str().to_string();
            let mut add = "ay ".to_string();
            add.insert(0, first_letter.to_ascii_lowercase());
            root.push_str(&add);
            pig_lating_s.push_str(&root);
        } else {
            let add = "hay ";
            let new = i.chars();
            let mut new = new.as_str().to_string();
            new.push_str(&add);
            pig_lating_s.push_str(&new);
        }
    }
    println!("{pig_lating_s}")
}
