pub struct Game {
	pub answer: String,
	pub guess: String,
    pub feedback: String
}
pub fn feedback_guess(game:&mut Game) {
    let mut ret = "⬛⬛⬛⬛⬛".to_string();
    let guess = &game.guess;
    let answer = &game.answer;
    if guess.len() != 5{
        panic!("feedback_guess received guess {} with length {}!!", &guess, guess.len());
    }
    if answer.len() != 5{
        panic!("feedback_guess received answer{} with length {}!!", &answer, answer.len());
    }
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
    game.feedback=ret;
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