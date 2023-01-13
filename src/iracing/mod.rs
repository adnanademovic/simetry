//! Support for iRacing.
//!
//! Use [`commands`] to send messages to iRacing.

pub mod commands;
mod flags;

pub use flags::{CameraFlag, CameraState};
