//use std::io;

fn main(){
	let mut choice = String::new();	
	choice = "Jojo".to_string();

	println!("Guess the number");
	println!("Plese enter the number");

	
	std::io::stdin()
		.read_line(&mut choice)
		.expect("Failed to read line");

	println!("Your number is: {}", choice);
}

