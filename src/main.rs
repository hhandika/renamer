// Heru Handika
// 16 January 2021

mod cli;
mod input;
mod output;

fn main() {
    let version = "0.1.0";

    cli::get_cli(&version);
    
    println!("Thank you for using renamer v{}", &version);
    
}


