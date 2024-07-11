use crate::utils::handle_choice;
use crate::utils::print_wrapped_text;
mod matches;
use matches::Matches;
mod lighter;
use lighter::Lighter;

pub struct Hall {
  
}

impl Hall {
  pub fn new() -> Self {
      Hall {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    const HALL_MATCHES_OR_LIGHTER: [&str; 2] = ["MATCHES", "LIGHTER"];
    print_wrapped_text("\n\nYou get out of bed and step into the hall. The hall is very dark, but you see a small table in the dim light. You step forward. On the table is a box of MATCHES and a LIGHTER. You can only pick up one item. Which item do you choose?");
    let choice = handle_choice(&HALL_MATCHES_OR_LIGHTER).to_string();
    if choice == "MATCHES" {
      let matches = Matches::new();
      matches.enter();
    }
    else {
      let lighter = Lighter::new();
      lighter.enter();
    }
  }
}