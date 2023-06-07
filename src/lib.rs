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

  pub fn guess(&mut self, word: String) -> Result<(String, bool), GameError> {
    let mut response: String = String::from("");
    let mut end_game: bool = false;

    if self.guesses == Self::TOTAL_GUESS {
      response = String::from("You are out of guesses. The word was ") + &self.match_word;
      end_game = true;
    } else if word.len() != 5 {
      response = String::from("Enter a 5-letter guess.");
    } else if word == self.match_word {
      response = String::from("You won!");
      end_game = true;
    } else {
      self.guesses += 1;

      for index in 0..5 {
        let guess_char = word.chars().nth(index).unwrap();
        // Guessed char in correct position
        if guess_char == self.match_word.chars().nth(index).unwrap() {
          response.push(guess_char);
        // Guessed char is correct but in wrong position
        } else if self.match_word.contains(guess_char) {
          response.push('#');
        // Guessed char not in correct word
        } else {
          response.push('_');
        }
      }

    }
    return Ok((response, end_game));
  }
}