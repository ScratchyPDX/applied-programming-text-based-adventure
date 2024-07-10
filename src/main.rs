use std::io::{self, Write}; // Import Write to flush stdout

// imports from other modules
mod utils;
use utils::handle_choice;
use utils::clear_screen;
use utils::quit_game;

const GO_OR_STAY_LIST: [&str; 3] = ["GO", "STAY", "BACK"];
const HALL_MATCHES_OR_LIGHTER: [&str; 2] = ["MATCHES", "LIGHTER"];
const LEFT_RIGHT_OR_TEDDY_BEAR: [&str; 5] = ["LEFT", "RIGHT", "TEDDY BEAR", "BEAR", "TEDDY"];

fn main() {
    clear_screen();

    print!("Please enter your name: ");

    // Flush stdout to ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();

    // Create a variable to hold the user input
    let mut name = String::new(); 

    // Call the stdin function from the io module to get user input
    // Read a line of input into the `name` variable
    io::stdin().read_line(&mut name).expect("Failed to read line"); // Handle potential errors
    name = name.to_uppercase();

    println!("\nHello, {}. Welcome to The Nightmare!!", name.trim().to_uppercase()); 
    println!("Let's begin....\n");

    // Starting location
    bedroom(name);
}

fn bedroom(name: String) {
    println!("{}, you are in a bedroom", name);
    println!("It's the middle of the night, and you startled awake by a noise coming from outside the room. What do you do?? GO check out the noise or STAY in bed?");
    let choice = handle_choice(&GO_OR_STAY_LIST).to_string(); 
    if choice == "STAY" {
        quit_game();
    } else if choice == "BACK" {
        println!("You cannot go BACK from here.");
        bedroom(name);
    } else {
        hall(name);
    }
}

fn hall(name: String) {
    println!("{}, you are in a hall", name);
    println!("You see a door on the LEFT, a door on the RIGHT, and a TEDDY BEAR on the floor. Which do you choose? LEFT, RIGHT, or TEDDY BEAR?");
}
