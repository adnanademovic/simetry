//! Commands to control force feedback.
//!
//! Using this override blocks the menu setting.

use super::BROADCAST_FFBCOMMAND;

/// Sets force feedback override specified max force in Nm.
///
/// If given value is not positive, does nothing.
pub fn set(max_force: f32) {
    if max_force > 0.0 {
        set_any(max_force);
    }
}

/// Clears force feedback override and unlocks the menu setting.
pub fn clear() {
    set_any(-1.0)
}

fn set_any(amount: f32) {
    BROADCAST_FFBCOMMAND.run((0, amount))
}
