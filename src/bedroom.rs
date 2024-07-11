use crate::utils::quit_game;
use crate::utils::handle_choice;
mod hall;
use hall::Hall;

pub struct Bedroom {
  player_name: String
}

impl Bedroom {
  pub fn new(player_name: String) -> Self {
      // GO_OR_STAY_LIST = ["GO", "STAY", "BACK"];
      Bedroom { player_name }
  }

  pub fn enter(&self) {
    const GO_OR_STAY_LIST: [&str; 3] = ["GO", "STAY", "BACK"];
    println!("{}, you are in a bedroom", self.player_name);
    println!("It's the middle of the night, and you startled awake by a noise coming from outside the room. What do you do?? GO check out the noise or STAY in bed?");
    let choice = handle_choice(&GO_OR_STAY_LIST).to_string(); 
    if choice == "STAY" {
        quit_game();
    } else if choice == "BACK" {
        println!("You cannot go BACK from here.");
        self.enter();
    } else {
        let hall = Hall::new();
        hall.enter();
    }
  }
}