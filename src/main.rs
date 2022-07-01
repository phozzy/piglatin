use std::io;

fn get1word() -> String {
    loop {
        let mut word = String::new();
        println!("Please enter your word:");
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read the user's input.");
        break match word.trim().parse() {
            Ok(str) => str,
            Err(_) => {
                println!("Wrong value! {}", word);
                continue;
            },
        };
    }
}

fn piglatin(word: &String) -> String {
    let vowels = "aeoui";
    let mut result: String = String::new();
    let mut ending = String::from("hay");
    let mut first_char = true;
    for char in word.chars() {
        if first_char {
            first_char = false;
            if !vowels.contains(char) {
                ending = String::from(char) + &String::from("ay");
                continue;
            }
        }
        result += &String::from(char);
    }
    result + &ending
}

fn main() {
    let word = get1word();
    let pigword = piglatin(&word);
    println!("{}", pigword);
}