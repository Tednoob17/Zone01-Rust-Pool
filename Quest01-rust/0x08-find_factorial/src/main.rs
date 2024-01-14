
/*
*recursive
*/

/*
pub fn factorial(num: u64) -> u64 {
	let mut fact: u64 = 0;

	if num == 0 || num == 1 {
		return 1;
	}
	else {
		return num * factorial(num - 1);
	}

}
*/

/*
* iterative
*/

pub fn factorial(num: u64) -> u64 {
	let mut fact: u64 = 1;
	if num < 2 { 
		return 1;
	}
	else {
		for x in 1..=num {
			fact *= x;	
		}
		return fact;
	}

}

//use find_factorial::factorial;

fn main() {
    println!("The factorial of 0 = {}", factorial(0));
    println!("The factorial of 1 = {}", factorial(1));
    println!("The factorial of 5 = {}", factorial(5));
    println!("The factorial of 10 = {}", factorial(10));
    println!("The factorial of 19 = {}", factorial(19));
}

