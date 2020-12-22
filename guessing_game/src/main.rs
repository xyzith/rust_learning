use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the numer!");
		let mut rng = rand::thread_rng();
		let n1: u32 = rng.gen_range(0, 10);

	loop {
		println!("Please input your guess.(0~9)");
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

//		println!("The number is: {}", n1);
		println!("You guessed: {}", guess);

	//	guess.pop(); // remove linebreak

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&n1) {
			Ordering::Less => println!("low"),
			Ordering::Greater => println!("high"),
			Ordering::Equal => {
				println!("hit, The number is: {}", n1);
				break;
			}
		}
	}
}
