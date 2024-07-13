use crate::utils::*;
mod matches;
use matches::Matches;
mod lighter;
use lighter::Lighter;
use colored::*;

pub struct Hall {
  
}

impl Hall {
  pub fn new() -> Self {
      Hall {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    const MATCHES_OR_LIGHTER: [&str; 2] = ["MATCHES", "LIGHTER"];
    print_wrapped_text(
      &format!("{} {} {} {}{}", 
        "You get out of bed and step into the hall. The hall is very dark, but you see a small table in the dim light. You step forward. On the table is a box of", 
        MATCHES_OR_LIGHTER[0].green(), 
        "and a", 
        MATCHES_OR_LIGHTER[1].green(),  
        ". You can only pick up one item. Which one do you choose?"));
    let choice = handle_choice(&MATCHES_OR_LIGHTER).to_string();
    if choice == "MATCHES" {
      clear_screen();
      let matches = Matches::new();
      matches.enter();
    }
    else {
      clear_screen();
      let lighter = Lighter::new();
      lighter.enter();
    }
  }
}