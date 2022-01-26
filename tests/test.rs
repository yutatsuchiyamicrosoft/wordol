
#[test]
fn sanity() {
    assert_eq!(4, 2 + 2);
}

#[test]
fn correct_answer() {
    let mut game = miniproj::Game{
    answer: "hello".to_string(),
    guess: "hello".to_string(),
    feedback: "".to_string()
    };
    miniproj::feedback_guess(&mut game);
    assert_eq!(game.feedback, "🟩🟩🟩🟩🟩");
}

#[test]
fn one_yellow_answer() {
    let mut game = miniproj::Game{
    answer: "hello".to_string(),
    guess: "great".to_string(),
    feedback: "".to_string()
    };
    miniproj::feedback_guess(&mut game);
    assert_eq!(game.feedback, "⬛⬛🟨⬛⬛");
}

#[test]
#[should_panic(expected = "feedback_guess received answer hell with length 4!!")]
fn answer_wrong_length() {
    let mut game = miniproj::Game{
    answer: "hell".to_string(),
    guess: "hello".to_string(),
    feedback: "".to_string()
    };
    miniproj::feedback_guess(&mut game);
}

#[test]
#[should_panic(expected = "feedback_guess received guess hell with length 4!!")]
fn guess_wrong_length() {
    let mut game = miniproj::Game{
    answer: "hello".to_string(),
    guess: "hell".to_string(),
    feedback: "".to_string()
    };
    miniproj::feedback_guess(&mut game);
}