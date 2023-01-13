//! Commands to control if telemetry is recorded.

use super::BROADCAST_TELEM_COMMAND;

const TELEM_COMMAND_STOP: u16 = 0;
const TELEM_COMMAND_START: u16 = 1;
const TELEM_COMMAND_RESTART: u16 = 2;

/// Starts telemetry recording.
pub fn start() {
    BROADCAST_TELEM_COMMAND.run(TELEM_COMMAND_START)
}

/// Stops telemetry recording.
pub fn stop() {
    BROADCAST_TELEM_COMMAND.run(TELEM_COMMAND_STOP)
}

/// Restarts telemetry recording.
pub fn restart() {
    BROADCAST_TELEM_COMMAND.run(TELEM_COMMAND_RESTART)
}
