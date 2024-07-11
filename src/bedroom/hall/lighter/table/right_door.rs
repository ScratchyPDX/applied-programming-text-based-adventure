use crate::utils::quit_game;
use crate::utils::get_is_door_locked;
use crate::bedroom::hall::lighter::table::Table;

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
      println!("\n\nYou step to the RIGHT door and check the door knob. LOCKED! You return back to the table");
      let table = Table::new();
      table.enter();
    }
    else {
        println!("\n\nYou step to the RIGHT door and checks the door knob. LOCKED! You use the key and it unlocks the door. You slowly turn the knob and it opens! You step through and into freedom ...");
        println!("\n\n...The sound was coming from inside the house!!!!");
        println!("\n\nThanks for playing! Goodbye!");
        quit_game();
    }
  }
}