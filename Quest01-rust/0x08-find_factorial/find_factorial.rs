
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

