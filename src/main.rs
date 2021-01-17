// Heru Handika
// 16 January 2021

mod cli;
mod finder;
mod writer;
mod renamer;

use std::time::Instant;

fn main() {
    let version = "0.1.0";

    let tnow = Instant::now();
    cli::get_cli(&version);
    let elapsed = tnow.elapsed(); 
    
    println!("Execution time: {:?}", elapsed);
    println!("Thank you for using renamer v{} 🙏", &version);
    
}


