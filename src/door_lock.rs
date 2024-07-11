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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lock_is_locked() {
        let lock = DoorLock::new();
        assert!(lock.get_is_locked(), "New lock should be locked by default.");
    }

    #[test]
    fn get_is_locked_returns_correct_status() {
        let lock = DoorLock::new();
        // Initially, the lock is locked
        assert!(lock.get_is_locked(), "Lock should initially be locked.");

        // Change the lock status to unlocked
        lock.set_is_locked(false);
        assert!(!lock.get_is_locked(), "Lock should be unlocked after calling set_is_locked(false).");
    }

    #[test]
    fn set_is_locked_updates_lock_status() {
        let lock = DoorLock::new();

        // Unlock the lock
        lock.set_is_locked(false);
        assert!(!lock.get_is_locked(), "Lock should be unlocked after calling set_is_locked(false).");

        // Lock the lock again
        lock.set_is_locked(true);
        assert!(lock.get_is_locked(), "Lock should be locked after calling set_is_locked(true).");
    }
}