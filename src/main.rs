pub use std::fs;
pub use std::collections::HashMap;
pub use std::io;
pub use rand::Rng;
pub use ascii::{AsciiChar, AsciiStr, AsciiString};
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
        println!("Please enter your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Expected a string input.");
        guess.pop();
        let guess_as_str = guess.as_str();
        if guess.len() != 5{
            println!("Received word {} with length {}. Please guess a 5 letter word.", &guess, guess.len());
            continue;
        }
        if !words_dict.contains_key(&guess_as_str){
            println!("Received word {} not a valid word.", &guess);
            continue;
        }
        let feedback = feedback_guess(&guess, &answer);
        println!("{}", feedback);
        if guess.eq(&answer){
            println!("🥳Congratulations!🥳 You won in {} guesses.", guess_count);
            break;
        }
        guess_count += 1;
    }
}
fn feedback_guess(guess: & String, answer: & str)-> String {
    let mut ret = "⬛⬛⬛⬛⬛".to_string();
    for i in 0..5{
        if guess.chars().nth(i).unwrap() == answer.chars().nth(i).unwrap(){
            ret = turn_ithchar_green(&mut ret, i);
        }
    }
    for i in 0..5{
        if ret.chars().nth(i).unwrap() == '🟩'{
            continue;
        }
        let mut count_in_ans = 0;
        let mut count_in_guess = 0;
        for j in 0..5{
            if guess.chars().nth(i).unwrap() == answer.chars().nth(j).unwrap() && guess.chars().nth(j).unwrap() != answer.chars().nth(j).unwrap(){
                count_in_ans += 1;
            }
        }
        for j in 0..i{
            if guess.chars().nth(i).unwrap() == guess.chars().nth(j).unwrap() && guess.chars().nth(j).unwrap() != answer.chars().nth(j).unwrap(){
                count_in_guess += 1;
            }
        }
        // Only mark the first count_in_ans characters as yellow.
        if count_in_ans > count_in_guess{
            ret = turn_ithchar_yellow(&mut ret, i);
        }
    }
    return ret;
}
fn turn_ithchar_green(ret: &mut String, i: usize) -> String{
    let mut ret1 = String::new();

    {
        for j in 0..5{
            if i != j {
                ret1.push(ret.chars().nth(j).unwrap());
            }
            else {
                ret1.push('🟩');
            }
        }
    }
    return ret1;
}
fn turn_ithchar_yellow(ret: &mut String, i: usize) -> String{
    let mut ret1 = String::new();

    {
        for j in 0..5{
            if i != j {
                ret1.push(ret.chars().nth(j).unwrap());
            }
            else {
                ret1.push('🟨');
            }
        }
    }
    return ret1;
}