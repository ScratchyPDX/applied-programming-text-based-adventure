use std::io::{self, Write}; // Import Write to flush stdout
use std::sync::Mutex;

// imports from other modules
mod utils;
use utils::handle_choice;
use utils::clear_screen;
use utils::quit_game;

const GO_OR_STAY_LIST: [&str; 3] = ["GO", "STAY", "BACK"];
const HALL_MATCHES_OR_LIGHTER: [&str; 2] = ["MATCHES", "LIGHTER"];
const LEFT_RIGHT_OR_TEDDY_BEAR: [&str; 5] = ["LEFT", "RIGHT", "TEDDY BEAR", "BEAR", "TEDDY"];
static IS_DOOR_LOCKED: Mutex<bool> = Mutex::new(true);

fn main() {
    clear_screen();

    // set the initial value of the door as locked
    {
        let mut door_locked = IS_DOOR_LOCKED.lock().unwrap();
        *door_locked = true;
    }
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
    println!("Let's begin....\n");

    // Starting location
    bedroom(player_name);
}

fn bedroom(player_name: String) {
    println!("{}, you are in a bedroom", player_name);
    println!("It's the middle of the night, and you startled awake by a noise coming from outside the room. What do you do?? GO check out the noise or STAY in bed?");
    let choice = handle_choice(&GO_OR_STAY_LIST).to_string(); 
    if choice == "STAY" {
        quit_game();
    } else if choice == "BACK" {
        println!("You cannot go BACK from here.");
        bedroom(player_name);
    } else {
        hall();
    }
}

fn hall() {
    println!("\n\nYou get out of bed and step into the hall. The hall is very dark, but you see a small table in the dim light. You step forward. On the table is a box of MATCHES and a LIGHTER. You can only pick up one item. Which item do you choose?");
    let choice = handle_choice(&HALL_MATCHES_OR_LIGHTER).to_string();
    if choice == "MATCHES" {
        matches();
    }
    else {
        lighter();
    }
}

fn matches() {
    println!("\n\nYou pick up the box of MATCHES, and as you slides the box open, a giant spider crawls out. It immediately bites you on the back of the hand, injecting a deadly venom. You are dead!! Too bad. Better luck next time. Bwahahaha!\n\n");
    quit_game();
}

fn lighter() {
    println!("\n\nYou pick up the LIGHTER. With a <Click> <Click> the LIGHTER comes to life and the flame emits a faint glow.");
    table();
}

fn table() {
    println!("\n\nWith a feeling of uneasiness, you step forward and sees a small table.  There's a TEDDY BEAR on the it, and it's flanked by two doors - one on the LEFT and another on the RIGHT. Which do you choose?");
    let choice = handle_choice(&LEFT_RIGHT_OR_TEDDY_BEAR).to_string(); 
    println!("You chose '{}'", choice);
    if choice == "LEFT" {
        left_door();
    }
    else if choice == "RIGHT" {
        right_door();
    }
    else {
        teddy_bear();
    }
}

fn left_door() {
    println!("\n\nYou steps to the LEFT door and checks the door knob. It turns freely, and you slowly opens the door to the room beyond is dark. You raise the LIGHTER to illuminate the space beyond. Just as you do, a hideous monster jumps out from the inky depths and yells, 'BOO!' You die of fright!  To bad. Better luck next time. Bwahahaha!");
    quit_game();
}

fn right_door() {
    println!("right_door: 1");
    let mut is_locked: bool;
    {
        let door_locked = IS_DOOR_LOCKED.lock().unwrap();
        is_locked = *door_locked;
    }
    println!("right_door: 2");
    if is_locked {
        println!("\n\nYou step to the RIGHT door and check the door knob. LOCKED! You return back to the table");
        table();
    }
    else {
        println!("\n\nYou step to the RIGHT door and checks the door knob. LOCKED! You use the key and it unlocks the door. You slowly turn the knob and it opens! You step through and into freedom ...");
        println!("...The sound was coming from inside the house!!!!");
        println!("Thanks for playing! Goodbye!");
        quit_game();
    }
}

fn teddy_bear() {
    {
        let mut door_locked = IS_DOOR_LOCKED.lock().unwrap();
        *door_locked = false;
    }
    println!("\n\nWith apprehension you approach the table and pick up the TEDDY BEAR. You feel something attached to its back and turns the TEDDY BEAR over. There's a key attached and you removes the key. Having made a new discovery, you return to the table");
    table();
}