use crate::utils::handle_choice;
use crate::utils::print_wrapped_text;
mod left_door;
use left_door::LeftDoor;
mod right_door;
use right_door::RightDoor;
mod teddy_bear;
use teddy_bear::TeddyBear;

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
    print_wrapped_text("\n\nWith a feeling of uneasiness, you step forward and sees a small table.  There's a TEDDY BEAR on the it, and it's flanked by two doors - one on the LEFT and another on the RIGHT. Which do you choose?");
    let choice = handle_choice(&LEFT_RIGHT_OR_TEDDY_BEAR).to_string(); 
    if choice == "LEFT" {
      let left_door = LeftDoor::new();
      left_door.enter();
    }
    else if choice == "RIGHT" {
      let right_door = RightDoor::new();
      right_door.enter();
    }
    else {
      let teddy_bear = TeddyBear::new();
      teddy_bear.enter();
    }
  }
}