use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct DoorLock {
  is_locked: Mutex<bool>,
}

impl DoorLock {
  // Constructor function to create a new DoorLock
  pub fn new() -> Self {
      DoorLock {
          is_locked: Mutex::new(true), // Default to true, indicating locked
      }
  }

  // Method to get the current lock status
  pub fn get_is_locked(&self) -> bool {
      let lock = self.is_locked.lock().unwrap();
      *lock
  }

  // Method to set the lock status
  pub fn set_is_locked(&self, value: bool) {
      let mut lock = self.is_locked.lock().unwrap();
      *lock = value;
  }
}

// Create a global instance of DoorLock using lazy_static
lazy_static! {
  pub static ref GLOBAL_DOOR_LOCK: DoorLock = DoorLock::new();
}