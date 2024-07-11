use crate::utils::quit_game;
use crate::utils::print_wrapped_text;

pub struct LeftDoor {
  // Hall-specific fields
}

impl LeftDoor {
  pub fn new() -> Self {
      LeftDoor {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    print_wrapped_text("\n\nYou steps to the LEFT door and checks the door knob. It turns freely, and you slowly opens the door to the room beyond is dark. You raise the LIGHTER to illuminate the space beyond. Just as you do, a hideous monster jumps out from the inky depths and yells, 'BOO!' You die of fright!  To bad. Better luck next time. Bwahahaha!");
    quit_game();
  }
}