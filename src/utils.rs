use std::io::{self, Write}; // Import Write to flush stdout
use std::process::Command;
use std::process;

pub fn clear_screen() {
  Command::new("clear").status().unwrap();
}

pub fn handle_choice(choice_list: &[&str]) -> String {
  let mut choice = String::new();

  loop {
      print!("> ");
      io::stdout().flush().unwrap();
      choice.clear();
      io::stdin().read_line(&mut choice).expect("Failed to read line");
      let choice = choice.trim().to_uppercase();
      println!("You chose '{}'", choice);
      if choice == "QUIT" {
          quit_game();
          break; 
      } else if !choice_list.contains(&choice.as_str()) {
          println!("\nThat's not one of the choices. Your choices are: {:?}", choice_list);
      } else {
          break;
      }
  }
  return choice.trim().to_uppercase().to_string();
}

pub fn quit_game() {
  println!("Quitting game...");
  // Exit the program with a success code
  process::exit(0); 
}