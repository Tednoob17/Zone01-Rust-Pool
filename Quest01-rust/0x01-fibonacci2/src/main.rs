/**
*main : Test fibonacci2 function
*/

pub fn fibonacci(n: u32) -> u32 {
    let  value: u32;

    if  n < 2 {
    	value = 1;
    }
    else {
    	 value = fibonacci(n - 1) + fibonacci(n - 2);
    }	 
    value
 }

//use fibonacci2::fibonacci;

fn main() {
    println!("The element in the position {} in fibonacci series is {}",2, fibonacci(2));
    println!("The element in the position {} in fibonacci series is {}",4, fibonacci(4));
    println!("The element in the position {} in fibonacci series is {}",22, fibonacci(22));
    println!("The element in the position {} in fibonacci series is {}", 20, fibonacci(20));
}
