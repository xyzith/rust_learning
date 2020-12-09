use std::io;
use rand::Rng;

fn main() {
	println!("Guess the numer!");

	println!("Please input your guess.(0~9)");
	let mut rng = rand::thread_rng();

	let n1: u8 = rng.gen_range(0, 10);
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("The number is: {}", n1);
	println!("You guessed: {}", guess);

	guess.pop(); // remove linebreak

	let guess_num: u8 = guess.parse::<u8>().unwrap();

	if n1 == guess_num {
		println!("You win.");
	} else {
		println!("You lose.");
	}

}
