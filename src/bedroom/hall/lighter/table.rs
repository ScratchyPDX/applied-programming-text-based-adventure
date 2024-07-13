use crate::utils::*;
mod left_door;
use left_door::LeftDoor;
mod right_door;
use right_door::RightDoor;
mod teddy_bear;
use teddy_bear::TeddyBear;
use colored::*;

pub struct Table {
  
}

impl Table {
  pub fn new() -> Self {
    Table {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    const LEFT_RIGHT_OR_TEDDY_BEAR: [&str; 5] = ["LEFT", "RIGHT", "TEDDY BEAR", "BEAR", "TEDDY"];
    print_wrapped_text(
      &format!("{} {} {} {} {} {}{}", 
        "\nWith a feeling of uneasiness, you step forward and see a small table. There's a", 
        LEFT_RIGHT_OR_TEDDY_BEAR[2].green(), 
        "on the it, and it's flanked by two doors - one on the",
        LEFT_RIGHT_OR_TEDDY_BEAR[0].green(),
        "and another on the",
        LEFT_RIGHT_OR_TEDDY_BEAR[1].green(),
        ". Which do you choose?"
      ));
    let choice = handle_choice(&LEFT_RIGHT_OR_TEDDY_BEAR).to_string(); 
    if choice == "LEFT" {
      clear_screen();
      let left_door = LeftDoor::new();
      left_door.enter();
    }
    else if choice == "RIGHT" {
      clear_screen();
      let right_door = RightDoor::new();
      right_door.enter();
    }
    else {
      clear_screen();
      let teddy_bear = TeddyBear::new();
      teddy_bear.enter();
    }
  }
}