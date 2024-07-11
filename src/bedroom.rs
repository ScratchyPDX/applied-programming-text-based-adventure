use crate::utils::*;
mod hall;
use hall::Hall;

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
        colorize_default(GO_OR_STAY_LIST[0]), 
        "check out the noise or", 
        colorize(GO_OR_STAY_LIST[1], "red"),  
        "in bed?"));
 
    let choice = handle_choice(&GO_OR_STAY_LIST).to_string(); 

    if choice == "STAY" {
      print_wrapped_text(
        &format!("\n{}{}", 
          "Ok, stay in bed....", 
          italic(&colorize("CHICKEN!!!", "red"))));
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