use std::io;

fn main() {
   let mut i = 0;
   let mut guess = String::new();

   loop {
   println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place.\
   	        What am I?");
   io::stdin().read_line(&mut guess)
   	.expect("Failed to read the line");
   i = i + 1;
   if guess == "e" {
      break;
   }
};
   println!("Number of trials: {}", i);

}
