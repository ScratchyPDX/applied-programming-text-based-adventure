use crate::utils::print_wrapped_text;
mod table;
use table::Table;
use colored::*;

pub struct Lighter {

}

impl Lighter {
  pub fn new() -> Self {
    Lighter {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    print_wrapped_text(
      &format!("{} {}{} {} {} {}.", 
        "\n\nYou pick up the", 
        "LIGHTER".green(), 
        ". With a <Click> <Click> the",
        "LIGHTER".green(),
        "comes to life and the flame emits a faint",
        "glow".yellow().bold().on_color("red")
        ));
    let table = Table::new();
    table.enter();
  }
}