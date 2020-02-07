use std::io;
fn get_string() -> String {
    loop {
        let mut word = String::new();
        println!("Please enter your word:");
        io::stdin().read_line(&mut word)
            .expect("Failed to read line");
        break match word.trim().parse() {
            Ok(str) => str,
            Err(_) => {
                println!("Wrong value! {}", word);
                continue;
            },
        };
    }
}
fn convert_string(word: &String) -> String {
    let vowel = ['a', 'e', 'i', 'o', 'u'];
    let mut counter: u32 = 0;
    let mut result = String::new();
    let mut ending = String::new();
    for character in word.chars() {
        counter += 1;
        if counter == 1 {
            if vowel.contains(&character) {
                ending = String::from("hay");
                result = character.to_string();
            } else {
                ending = character.to_string() + "ay";
                result = "".to_string();
            };
        } else {
            result += &character.to_string();
        };
    };
    result + &ending
}
fn main() {
    let word = get_string();
    println!("converted to: {}", convert_string(&word));
}
