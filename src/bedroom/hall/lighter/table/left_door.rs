use crate::utils::quit_game;
use crate::utils::print_wrapped_text;
use colored::*;
use std::{thread, time::Duration};
use std::io::{self, Write};

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
    print_wrapped_text(
      &format!("{} {} {}", 
        "\n\nYou steps to the", 
        "LEFT".green(), 
        "door and checks the door knob. It turns freely, and you slowly open the door..."));

        thread::sleep(Duration::from_secs(4));
        print!("{}", "\nC ".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "R ".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "E ".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "A ".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "K ".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "!".red().bold().italic());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(2));

        print_wrapped_text("\n\n...to the pitch-black room beyond.");
        thread::sleep(Duration::from_secs(4));

        print_wrapped_text(
          &format!("{} {} {} {} {} {}", 
            "\n\nYou raise the", 
            "LIGHTER".green(), 
            "to illuminate the space beyond. Just as you do, a hideous",
            "MONSTER".green().bold(),
            "jumps out from the inky depths and yells,",
            "'BOO!'".yellow().bold().italic()));

        thread::sleep(Duration::from_secs(3));
        print!("{}", "\nYOU".red().bold());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", " ARE ".red().bold());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("{}", "DEAD!!".red().bold());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(4));

        print_wrapped_text("\n\nBwahahaha! You die of fright! To bad... Better luck next time.");
        quit_game();
  }
}