use std::io;

fn main() -> io::Result<()> {
   let mut count = 0;
   let save = "The letter e";
   
   loop {
   let mut answer = String::new();
   println!("I am the beginning of the end, and the end of time and space. \
   	       I am essential to creation, and I surround every place. What am I? :");
   count +=1;
   io::stdin().read_line(&mut answer).unwrap();	

   if answer.trim() == save  {
	break;
     }
  };

   println!("Number of trials: {count}");
   Ok(())
}

