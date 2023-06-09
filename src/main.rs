use std::io;
use wordle::{words, Game};

fn main() -> std::io::Result<()> {
  println!("{}", "------ Wordle ------");

  let match_word: String = words::random_word();

  let mut game: Game = Game::new(match_word);

  loop {
    let mut guess: String = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: String =  match guess.trim().to_lowercase().parse() {
      Ok(guess) => guess,
      Err(_) => continue,
    };

    let results: bool = game.guess(&guess).unwrap();

    if results {
      break;
    }
  };

  Ok(())
}