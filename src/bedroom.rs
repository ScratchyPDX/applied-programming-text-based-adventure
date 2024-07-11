use crate::utils::*;
mod hall;
use hall::Hall;
use colored::*;
use std::{thread, time::Duration};
use std::io::{self, Write};

pub struct Bedroom {
  player_name: String
}

impl Bedroom {
  pub fn new(player_name: String) -> Self {
      Bedroom { player_name }
  }

  pub fn enter(&self) {
    const GO_OR_STAY_LIST: [&str; 3] = ["GO", "STAY", "BACK"];
    println!("{}, you are in a bedroom.", self.player_name);
 
    print_wrapped_text(
      &format!("\n{} {} {} {} {}", 
        "It's the middle of the night, and you startled awake by a noise coming from outside the room. What do you do??", 
        GO_OR_STAY_LIST[0].red(), 
        "check out the noise or", 
        GO_OR_STAY_LIST[1].red(),  
        "in bed?"));
 
    let choice = handle_choice(&GO_OR_STAY_LIST).to_string(); 

    if choice == "STAY" {
      print!("\n\nOk, stay in bed....");
      io::stdout().flush().unwrap();
      thread::sleep(Duration::from_secs(2));
      println!("{}", "CHICKEN!!!".red().italic());
      thread::sleep(Duration::from_secs(2));
      quit_game();
    } 
    else if choice == "BACK" {
      println!("You cannot go BACK from here.");
      self.enter();
    } 
    else {
      let hall = Hall::new();
      hall.enter();
    }
  }
}