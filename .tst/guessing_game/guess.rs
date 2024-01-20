use std::io;

fn main(){
	let mut choice = String::new();	

	println!("Guess the number");
	println!("Plese enter the number");

	
	io::stdin()
		.read_line(&mut choice)
		.expect("Failed to read line");

	println!("Your number is: {}", choice);
}

