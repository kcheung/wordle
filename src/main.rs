use std::io;
use wordle::{words, Game};

fn main() {
  println!("{}", "------ Wordle ------");

  let match_word = words::random_word();

  let mut game = Game::new(match_word);

  loop {
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: String =  match guess.trim().to_lowercase().parse() {
      Ok(guess) => guess,
      Err(_) => continue,
    };

    let results = game.guess(guess).unwrap();

    println!("> {}", results.0);
    if results.1 {
      break;
    }
  }
}