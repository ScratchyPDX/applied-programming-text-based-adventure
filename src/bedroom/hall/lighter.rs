use crate::utils::print_wrapped_text;
mod table;
use table::Table;

pub struct Lighter {

}

impl Lighter {
  pub fn new() -> Self {
    Lighter {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    print_wrapped_text("\n\nYou pick up the LIGHTER. With a <Click> <Click> the LIGHTER comes to life and the flame emits a faint glow.");
    let table = Table::new();
    table.enter();
  }
}