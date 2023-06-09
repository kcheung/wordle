use std::{io::{stdout, Write, self}};

use crossterm::{
  style::{Color, self},
  queue,
  execute,
  terminal,
  cursor, event::{Event, KeyEvent, self, KeyCode, KeyEventKind},
};

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

  pub fn run<W>(mut self, w: &mut W) -> io::Result<()>
  where
    W: io::Write,
  {
    execute!(
      w,
      terminal::EnterAlternateScreen,
      cursor::SetCursorStyle::BlinkingUnderScore,
    )?;
    terminal::enable_raw_mode()?;

    loop {
      let guess_string = self.read_line(w);
      if self.guess(&guess_string.unwrap())? {
        break;
      }
      execute!(w, cursor::MoveToNextLine(1))?;
    }

    // Close game after any key press.
    // TODO: Move into method
    loop {
      if let Ok(Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        kind: KeyEventKind::Press,
        modifiers: _,
        state: _,
      })) = event::read()
      {
        execute!(
          w,
          style::ResetColor,
          cursor::Show,
          terminal::LeaveAlternateScreen
        )?;
        return terminal::disable_raw_mode();
      }
    }
  }

  // Build guess by adding each key press into a single entry once "Enter" is pressed
  fn read_line<W>(&self, w: &mut W) -> io::Result<String>
  where
    W: io::Write,
  {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
      match code {
        KeyCode::Enter => {
          execute!(
            w,
            cursor::MoveLeft(5)
          )?;
          break;
        }
        KeyCode::Backspace => {
          line.pop();
          execute!(
            w,
            cursor::MoveLeft(1),
            style::Print(' '),
            cursor::MoveLeft(1),
          )?;
        }
        KeyCode::Char(c) => {
          line.push(c);
          execute!(
            w,
            style::Print(c)
          )?;
        }
        _ => {}
      }
    }

    Ok(line)
  }

  // Given a guess string, check if the guess is a match or has any correct chars.
  // This `queue`s Commands to be executed once the io:Write object is flushed.
  // TODO: Clean up
  pub fn guess(&mut self, word: &String) -> io::Result<bool> {
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

      if self.guesses == Self::TOTAL_GUESS {
        end_game = true;
        response = String::from(format!("\nYou are out of guesses. The word was {}\n", self.match_word));
      } else {
        response = String::from("\n");
      }

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
          style::SetForegroundColor(char_color),
          style::Print(guess_char.to_string()),
        )?;
      }

      queue!(
        s,
        style::ResetColor,
      )?;
    }

    queue!(
      s,
      style::SetForegroundColor(color),
      style::Print(response),
      style::ResetColor,
      cursor::MoveToNextLine(1),
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
    assert_eq!(results, true);
  }
}