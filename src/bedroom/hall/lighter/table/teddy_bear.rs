use crate::utils::set_is_door_locked;
use crate::utils::print_wrapped_text;
use crate::bedroom::hall::lighter::table::Table;

pub struct TeddyBear {
  // Hall-specific fields
}

impl TeddyBear {
  pub fn new() -> Self {
    TeddyBear {
          // Initialize fields
      }
  }

  pub fn enter(&self) {
    set_is_door_locked(false);
    print_wrapped_text("\n\nWith apprehension you approach the table and pick up the TEDDY BEAR. You feel something attached to its back and turns the TEDDY BEAR over. There's a key attached and you removes the key. Having made a new discovery, you return to the table");
    let table = Table::new();
    table.enter();
  }
}