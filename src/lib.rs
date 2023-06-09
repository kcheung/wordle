use std::{io::{stdout, Write}};

use crossterm::{style::{Print, SetForegroundColor, Color, ResetColor}, queue};

pub mod words {
  use std::fs;
  use rand::Rng;

  const MAX: usize = 2309;
  const WORD_LENGTH: usize = 5;
  const WORD_OFFSET: usize = 6;
  const WORD_FILEPATH: &str = "words.txt";

  pub fn random_word() -> String {
    let word_index = rand::thread_rng().gen_range(0..MAX) * WORD_OFFSET;
    let words = fs::read_to_string(WORD_FILEPATH).unwrap();
    words[word_index..word_index+WORD_LENGTH].to_string()
  }
}

pub struct Game {
  pub match_word: String,
  pub guesses: i32,
  pub alphabet: Vec<char>
}

#[derive(Debug)]
pub enum GameError {
  GenericError,
}

impl Game {
  const TOTAL_GUESS: i32 = 6;

  pub fn new(match_word: String) -> Game {
    Game {
      match_word: match_word,
      guesses: 0,
      alphabet: vec![
        'a','b','c','d','e','f','g','h','i','j','k','l','m',
        'n','o','p','q','r','s','t','u','v','w','x','y','z',
      ],
    }
  }

  pub fn guess(&mut self, word: &String) -> Result<bool, std::io::Error> {
    let mut end_game: bool = false;
    let mut s: std::io::Stdout = stdout();
    let color: Color = Color::Blue;
    let response: String;

    if self.guesses == Self::TOTAL_GUESS {
      end_game = true;
      response = String::from(format!("You are out of guesses. The word was {}\n", self.match_word));
    } else if word.len() != 5 {
      response = String::from("Enter a 5-letter guess.\n");
    } else if *word == self.match_word {
      end_game = true;
      response = String::from("You won!\n");
    } else {
      self.guesses += 1;
      response = String::from("\n");

      for index in 0..5 {
        let guess_char: char = word.chars().nth(index).unwrap();
        let char_color: Color;

        if guess_char == self.match_word.chars().nth(index).unwrap() {
          // Guessed char in correct position
          char_color = Color::Green;
        } else if self.match_word.contains(guess_char) {
          // Guessed char is correct but in wrong position
          char_color = Color::Yellow;
        } else {
          // Guessed char not in correct word
          char_color = Color::Red;
        }

        queue!(
          s,
          SetForegroundColor(char_color),
          Print(guess_char.to_string()),
        )?;
      }

      queue!(
        s,
        ResetColor,
      )?;
    }

    queue!(
      s,
      SetForegroundColor(color),
      Print(response),
      ResetColor,
    )?;

    s.flush()?;

    Ok(end_game)
  }
}

#[cfg(test)]
mod tests {
  use crate::words;
  use super::*;

  #[test]
  fn test_random_word() {
    assert_eq!(words::random_word().len(), 5)
  }

  #[test]
  fn test_new() {
    let word = "atest";
    let game = Game::new(word.to_string());
    assert_eq!(game.guesses, 0);
    assert_eq!(game.match_word, word);
    assert_eq!(game.alphabet.len(), 26);
  }

  // Correctly guess in 3 turns
  #[test]
  fn test_guess_win_case() {
    let guessed_word_1 = String::from("stark");
    let guessed_word_2 = String::from("trash");
    let match_word = String::from("feast");
    let mut game = Game::new(match_word.clone());

    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_2).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&match_word).unwrap();
    assert_eq!(results, true);
  }

  // Wrongly guess 7 times
  #[test]

  fn test_guess_out_of_guesses_case() {
    let guessed_word_1 = String::from("stark");
    let match_word = String::from("feast");
    let mut game = Game::new(match_word.clone());

    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, false);
    let results = game.guess(&guessed_word_1).unwrap();
    assert_eq!(results, true);
  }
}