use std::io::{self, Write}; // Import Write to flush stdout
use std::process::Command;

fn main() {
    clear_screen();

    print!("Please enter your name: ");

    // Flush stdout to ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();

    // Create a variable to hold the user input
    let mut name = String::new(); 

    // Call the stdin function from the io module to get user input
    // Read a line of input into the `name` variable
    io::stdin() 
        .read_line(&mut name) 
        .expect("Failed to read line"); // Handle potential errors

    // Use trim to remove any newline characters
    println!("\nHello, {}. Welcome to The Nightmare!!", name.trim().to_uppercase()); 
    println!("Let's begin....\n");


}

fn clear_screen() {
    Command::new("clear").status().unwrap();
}   
