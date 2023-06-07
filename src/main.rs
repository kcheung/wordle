use wordle::words;

const TOTAL_GUESS: i32 = 6;

fn main() {
    println!("Wordle");

    let mut guesses = 0;

    let correct_word = words::todays_word();
    println!("{correct_word}");


    // loop {
    //     println!("Please input your guess.");

    //     if guesses == TOTAL_GUESS {
    //         println!("You are out of guess. The correct word was \"{correct_word}\"");
    //         break;
    //     }

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: String =  match guess.trim().to_lowercase().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guess: {guess}");

    //     if guess.len() != 5 {
    //         println!("Enter a 5-letter guess.")
    //     } else {
    //         guesses += 1;

    //         if guess == correct_word {
    //             println!("You win!");
    //             break;
    //         }

    //         let mut results = String::from("");
    //         for index in 0..5 {
    //             if guess.chars().nth(index).unwrap() == correct_word.chars().nth(index).unwrap() {
    //                 results.push(guess.chars().nth(index).unwrap());
    //             } else if correct_word.contains(guess.chars().nth(index).unwrap()) {
    //                 results.push('#');
    //             } else {
    //                 results.push('_');
    //             }
    //         }
    //         println!("{results}");
    //     }
    // }
}