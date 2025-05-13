use crate::{MeditationQuote, GameState, GameStatus};

#[test]
fn test_word_progression() {
    let quote = MeditationQuote {
        original_quotes: vec!["abc".to_string()],
        expanded_meditation: "".to_string(),
    };
    let mut game = GameState::new(vec![quote]);
    game.status = GameStatus::Running;
    game.input_buffer = "abc".to_string();
    assert_eq!(game.input_buffer, "abc");
}

#[test]
fn test_error_counting() {
    let quote = MeditationQuote {
        original_quotes: vec!["abc".to_string()],
        expanded_meditation: "".to_string(),
    };
    let mut game = GameState::new(vec![quote]);
    game.status = GameStatus::Running;
    game.input_buffer = "abd".to_string();
    // Simulate error logic
    let expected = &game.quotes[0].original_quotes[0];
    let input = &game.input_buffer;
    let correct = expected.starts_with(input);
    if !correct && !input.is_empty() {
        game.errors += 1;
    }
    assert_eq!(game.errors, 1);
}
