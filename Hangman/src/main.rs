#![allow(non_snake_case)]
#![allow(unused)]

use std::{io::*, collections::HashSet};
use rand::Rng;

// hangman object
struct Hangman {
    lives: i8,
    word: String,
    letters: Vec<char>,
    missing_characters: usize,
}

// create enums that have values that will be set and then matched against to display confirmation messages/end the game
enum Status {
    Playing,
    Won,
    Lost,
}

enum EnterLetterResult {
    Correct,
    Wrong,
    Repeat,
}

// add functionality to hangman object
impl Hangman {
    // this function checks whether the game has been won or lost or is still being played
    fn get_status(&self) -> Status {
        if self.lives == 0 {
            Status::Lost
        } else if self.missing_characters == 0 {
            Status::Won
        } else {
            Status::Playing
        }
    }

    // this function checks if the letter is in the word or not or already been entered
    fn insert_letter(&mut self, new_letter: char) -> EnterLetterResult {
        if self.letters.contains(&new_letter){
            return EnterLetterResult::Repeat
        }

        self.letters.push(new_letter);

        if self.word.contains(new_letter){
            self.missing_characters -= 1;
            EnterLetterResult::Correct
        } else {
            self.lives -= 1;
            EnterLetterResult::Wrong
        }
    }

    // this creates a string from the word replacing letters with underscore if they've not been entered yet
    fn get_hidden_word(&self) -> String {
        self.word
            .chars()
            .map(|c| if self.letters.contains(&c) { c } else { '_' })
            .collect::<String>()
    }

    // this function will display the word, lives left, and letters that have been typed
    fn display_gamestate(&self) {
        println!("----------------------");
        println!("Word: {}", self.get_hidden_word());
        println!("Lives: {}", self.lives);
        println!("Letters typed: {}", String::from_iter(&self.letters));
        println!("-----------------------");
    }
}

fn main() {

    let mut choice = "y";

    while !choice.eq_ignore_ascii_case("n"){
        
        println!("Welcome to Hangman");
        let mut generatedWord = generateWord();
        println!("Generated word: {}", generatedWord);

        // get individual letters of generated word
        let mut generatedWord_chars = HashSet::new();
        for c in generatedWord.chars() {
            generatedWord_chars.insert(c);
        }

        // define initial state of the game object
        let mut initial_game = Hangman {
            lives: 8,
            word: generatedWord,
            letters: Vec::new(),
            missing_characters: generatedWord_chars.len(),
        };

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
    let mut words = "drum, chief, business, boats, land, snake, stream, 
    science, field, ghost, poison, driving, rain";

    // Seperate each word with a comma
    let mut seperateWord: Vec<&str> = words.trim().split(',').collect();

    // Generates a random word from the vector and returns it
    let generate = rand::thread_rng().gen_range(0..seperateWord.len());
    
    return String::from(seperateWord[generate]);
}
