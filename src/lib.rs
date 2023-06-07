pub mod words {
  use std::fs;

  use rand::Rng;

  pub fn todays_word() -> String {
    let word_index = rand::thread_rng().gen_range(0..13) * 6;
    let words = fs::read_to_string("words.txt").unwrap();
    words[word_index..word_index+5].to_string()
  }
}