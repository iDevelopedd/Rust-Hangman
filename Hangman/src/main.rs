#![allow(non_snake_case)]
#![allow(unused)]

use std::io::*;
use rand::Rng;

fn main() {

    let mut choice = "y";

    while !choice.eq_ignore_ascii_case("n"){
        
        println!("Welcome to Hangman");
        let mut generatedWord = generateWord();
        println!("Generated word: {}", generatedWord);

        let mut input = String::new();

        println!("Enter a letter: ");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
               println!("You entered: {}", input);
            },
            Err(e) => println!("Error: {}", e)
        }; 

        // stdin.read_line(&mut user_input);
        
        let mut choice = String::new();
        println!("New game?(y/n)");
        match std::io::stdin().read_line(&mut choice) {
            Ok(_) => {
                println!("Options");
                
            }, 
            Err(e) => {
                println!("Error: {}", e);
            } 
        }
    }

}

// This function will generate a random word
fn generateWord() -> String {

    // Setup the random words
    let mut words = "drum, chief, business, books, land, snake, stream, 
    science, field, ghost, poison, driving, rain";

    // Seperate each word with a comma
    let mut seperateWord: Vec<&str> = words.trim().split(',').collect();

    // Generates a random word from the and returns it
    let generate = rand::thread_rng().gen_range(0..seperateWord.len());
    
    return String::from(seperateWord[generate]);
}
