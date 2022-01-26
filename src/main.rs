use std::fs;
use std::collections::HashMap;
use std::io;
use rand::Rng;
// Structure containing the current information about the game




fn main() {
    println!("Welcome to Wordol!");
    let mut guess_count = 1;
    let mut words_dict = HashMap::new();
    let filename = "words_list.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let words_split = contents.split_whitespace();
    let words_split_vec: Vec<&str> = words_split.collect();
    let answer = words_split_vec[rand::thread_rng().gen_range(0..words_split_vec.len())];
    for s in words_split_vec {
        // Insert my words into my dictionary for fast lookup.
        words_dict.insert(
            s,
            "True".to_string(),
        );
    }
    
    
    //println!("The answer is {}", &answer);
    loop {
        println!("Guess {}", guess_count);
        println!("Please enter your guess (in lower case)!");
        let mut guess1 = String::new();
        io::stdin().read_line(&mut guess1).expect("Expected a string input.");
        guess1.pop();
        let mut game = miniproj::Game{
            guess: guess1.clone(),
            answer: answer.to_string(),
            feedback: "".to_string()
        };
        let guess = guess1.clone();
        let guess_as_str = guess.as_str();
        if guess.len() != 5{
            println!("Received word {} with length {}. Please guess a 5 letter word.", &guess, guess.len());
            continue;
        }
        if !words_dict.contains_key(&guess_as_str){
            println!("Received word {} not a valid word.", &guess);
            continue;
        }
        miniproj::feedback_guess(&mut game);
        println!("{}", game.feedback);
        if guess.eq(&answer){
            println!("🥳Congratulations!🥳 You won in {} guesses.", guess_count);
            break;
        }
        guess_count += 1;
    }
}
