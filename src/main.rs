use std::io;
use wordle::{words, Game};

fn main() -> std::io::Result<()> {

  let match_word: String = words::random_word();

  let game: Game = Game::new(match_word);

  let mut stdout = io::stdout();
  game.run(&mut stdout)?;

  Ok(())
}