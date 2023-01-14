//! Support for iRacing.
//!
//! Use [`commands`] to send messages to iRacing.

mod client;
pub mod commands;
mod flags;
pub mod string_decoding;
mod util;

pub use client::Client;
pub use flags::{CameraFlag, CameraState};
