use crate::utils::quit_game;
use crate::utils::get_is_door_locked;
use crate::utils::print_wrapped_text;
use crate::bedroom::hall::lighter::table::Table;
use colored::*;
use std::{thread, time::Duration};

pub struct RightDoor {
  // Hall-specific fields
}

impl RightDoor {
  pub fn new() -> Self {
    RightDoor {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    if get_is_door_locked() {
      print_wrapped_text(
        &format!("{} {} {} {} {}", 
          "You step to the", 
          "RIGHT".green(), 
          "door and check the door knob....",
          "LOCKED!".red(),
          "You return back to the table."
        ));
      thread::sleep(Duration::from_secs(3));
      let table = Table::new();
      table.enter();
    }
    else {
        print_wrapped_text(
          &format!("{} {} {} {} {}", 
            "You step to the", 
            "RIGHT".green(), 
            "door and checks the door knob...",
            "LOCKED!".red(),
            "You use the key and it unlocks the door. You slowly turn the knob and it opens! You step through and into freedom..."
        ));
        thread::sleep(Duration::from_secs(8));
        print_wrapped_text("\n...The sound you heard was coming from inside the house!!!!");
        thread::sleep(Duration::from_secs(5));
        print_wrapped_text("\nThanks for playing! Goodbye!");
        thread::sleep(Duration::from_secs(4));
        quit_game();
    }
  }
}