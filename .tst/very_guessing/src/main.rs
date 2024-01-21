use std::io;
use rand::Rng;


fn main() {
	println!("Guess the number !");

	let secret_number = rand::thread_rng().gen_range(1..101);

	println!("The secret number is : {}", secret_number);

	println!("Please enter a number");

	let mut supposition = String::new();

	io::stdin()
		.read_line(&mut supposition)
		.expect("Failed to read the line");

	println!("Your number : {}", supposition);

}
