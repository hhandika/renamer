// Heru Handika
// 16 January 2021

mod cli;
mod checker;
mod finder;
mod parser;
mod renamer;
mod writer;


use std::time::Instant;

fn main() {
    let version = "0.3.5";

    let tnow = Instant::now();
    cli::get_cli(&version);
    let elapsed = tnow.elapsed(); 
    
    println!("\nExecution time: {:?}", elapsed);
    println!("Thank you for using renamer v{} 🙏", &version);
    
}


