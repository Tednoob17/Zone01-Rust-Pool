
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0    
}

//use temperature_conv::*;


fn main() {
	println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
	println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
}