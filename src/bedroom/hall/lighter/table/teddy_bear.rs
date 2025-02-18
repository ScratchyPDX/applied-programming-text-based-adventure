use crate::utils::set_is_door_locked;
use crate::utils::print_wrapped_text;
use crate::bedroom::hall::lighter::table::Table;
use colored::*;
use std::{thread, time::Duration};

pub struct TeddyBear {
  // Hall-specific fields
}

impl TeddyBear {
  pub fn new() -> Self {
    TeddyBear {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    set_is_door_locked(false);
    print_wrapped_text(
      &format!("{} {}{} {} {} {} {}", 
        "With apprehension you approach the table and pick up the", 
        "TEDDY BEAR".green(), 
        ". You feel something attached to its back and turns the",
        "TEDDY BEAR".green(),
        "over. There's a",
        "KEY".yellow(), 
        "attached and you removes it. Having made a new discovery, you return to the table."
    ));
    thread::sleep(Duration::from_secs(8));
    let table = Table::new();
    table.enter();
  }
}