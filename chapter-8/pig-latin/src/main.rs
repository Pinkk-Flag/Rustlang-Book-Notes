fn main() {
    {
        use std::io;

        let mut original_input = String::new();

        println!("Input what you wish to convert to pig latin: ");
        io::stdin().read_line(&mut original_input).expect("Failed to read input. Please try again!");

        convert(original_input);
    }

}
fn convert(string_input: String) {
    /*
    * How pig latin works
        WORDS WITH CONSONANTS AT THE BEGINNING:
            "hello" --> "ello-hay"
        ELSE (WITH VOWEL):
            "apple" --> "apple-hay"
    
    * My Logic
    Next, we need to iterate through each word in the list.
    For each word, check if it's first letter is a vowel, and do the necessary step.
    Else, do the necessary step.
     */

     let mut words: Vec<String> = string_input.split_whitespace().map(|string_input| string_input.to_string()).collect();

     println!("Words: {:?}", words);
     for word in &mut words {
         let first_char = word.to_lowercase().chars().next().unwrap();
 
         if is_vowel(first_char) {
             word.push_str("-hay");
             println!("{}", word);
         } else {
             let sliced_word = &word[1..];
             *word = sliced_word.to_owned() + "-" + &first_char.to_string() + "ay";
             println!("{}", word);
         }
     }
}

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}