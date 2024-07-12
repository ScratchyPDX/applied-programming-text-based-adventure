use crate::utils::quit_game;
use crate::utils::get_is_door_locked;
use crate::utils::print_wrapped_text;
use crate::bedroom::hall::lighter::table::Table;
use colored::*;

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
          "\n\nYou step to the", 
          "RIGHT".green(), 
          "door and check the door knob....",
          "LOCKED!".red(),
          "You return back to the table."
        ));
      let table = Table::new();
      table.enter();
    }
    else {
        print_wrapped_text(
          &format!("{} {} {} {} {}", 
            "\n\nYou step to the", 
            "RIGHT".green(), 
            "door and checks the door knob...",
            "LOCKED!".red(),
            "You use the key and it unlocks the door. You slowly turn the knob and it opens! You step through and into freedom..."
        ));
        print_wrapped_text("\n\n...The sound you heard was coming from inside the house!!!!");
        print_wrapped_text("\n\nThanks for playing! Goodbye!");
        quit_game();
    }
  }
}