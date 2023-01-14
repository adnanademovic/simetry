//! Support for iRacing.
//!
//! Use [`commands`] to send messages to iRacing.

mod client;
pub mod commands;
mod constants;
mod disk_client;
mod flags;
mod header;
mod session_info;
mod sim_state;
pub mod string_decoding;
mod util;

pub use client::Client;
pub use disk_client::DiskClient;
pub use flags::{CameraFlag, CameraState};
pub use header::Header;
pub use sim_state::SimState;
