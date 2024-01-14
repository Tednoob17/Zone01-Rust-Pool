
/**
*fibonacci2 - implement fibonacci series
*@n: index in fibonacci series
*Return: returns the `n`th number in the fibonacci serie
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
