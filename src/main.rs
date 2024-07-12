use std::io::{self, Write}; // Import Write to flush stdout

// imports from other modules
mod utils;
use utils::clear_screen;
use utils::set_is_door_locked;
mod door_lock;
mod bedroom;
use crate::bedroom::Bedroom;

fn main() {
    clear_screen();

    set_is_door_locked(true);

    print!("Please enter your name: ");

    // Flush stdout to ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();

    // Create a variable to hold the user input
    let mut player_name = String::new(); 

    // Call the stdin function from the io module to get user input
    // Read a line of input into the `player_name` variable
    io::stdin().read_line(&mut player_name).expect("Failed to read line"); // Handle potential errors
    player_name = player_name.trim().to_uppercase();

    println!("\nHello, {}. Welcome to The Nightmare!!", player_name); 
    println!("\nLet's begin....");

    // Starting location
    let bedroom = Bedroom::new(player_name);
    bedroom.enter();
}