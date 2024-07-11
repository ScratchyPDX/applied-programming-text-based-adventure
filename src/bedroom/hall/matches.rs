use crate::utils::quit_game;

pub struct Matches {
  // Hall-specific fields
}

impl Matches {
  pub fn new() -> Self {
    Matches {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    println!("\n\nYou pick up the box of MATCHES, and as you slides the box open, a giant spider crawls out. It immediately bites you on the back of the hand, injecting a deadly venom. You are dead!! Too bad. Better luck next time. Bwahahaha!\n\n");
    quit_game();
  }
}