//! Support for iRacing.
//!
//! Use [`commands`] to send messages to iRacing.

mod bit_field;
mod car_positions;
mod client;
pub mod commands;
mod constants;
mod disk_client;
pub mod flags;
mod header;
mod session_info;
mod sim_state;
mod var_data;

pub use bit_field::BitField;
pub use car_positions::CarPositions;
pub use client::Client;
pub use constants::{UNLIMITED_LAPS, UNLIMITED_TIME};
pub use disk_client::DiskClient;
pub use flags::{CameraFlag, CameraState};
pub use header::{DiskSubHeader, Header, VarHeader, VarHeaders, VarType};
pub use sim_state::SimState;
pub use var_data::{Value, VarData};
