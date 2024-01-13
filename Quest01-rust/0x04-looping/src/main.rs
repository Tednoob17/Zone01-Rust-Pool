use std::io;

fn main() {
   let mut answer = String::new();
   let mut count = 0;
   let save = "The letter e";

   loop {

   println!("I am the beginning of the end, and the end of time and space. \
   	       I am essential to creation, and I surround every place. What am I? :");
   count +=1;
   io::stdin()
	.read_line(&mut answer)
	.expect("Failed to read line");

   if answer == "e" {
	break;
     }
  };

   println!("Number of trials: {count}");
}