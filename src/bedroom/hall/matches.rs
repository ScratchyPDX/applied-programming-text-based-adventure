use crate::utils::quit_game;
use crate::utils::print_wrapped_text;
use colored::*;
use std::{thread, time::Duration};
use std::io::{self, Write};

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
    print_wrapped_text(
      &format!("{} {} {}", 
        "You pick up the box of", 
        "MATCHES".green(), 
        ", and as you slides the box open, a giant spider crawls out. It immediately bites you on the back of the hand, injecting a deadly venom.", 
        ));
    thread::sleep(Duration::from_secs(8));
    print!("{}", "\nYOU".red().bold());
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!("{}", " ARE ".red().bold());
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!("{}", "DEAD!!".red().bold());
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(4));
    println!("\n\nToo bad. Better luck next time. Bwahahaha!!!");
    quit_game();
  }
}