//! Commands to control pitstops.

use super::BROADCAST_PIT_COMMAND;

const PIT_COMMAND_CLEAR: u16 = 0;
const PIT_COMMAND_WS: u16 = 1;
const PIT_COMMAND_FUEL: u16 = 2;
const PIT_COMMAND_LF: u16 = 3;
const PIT_COMMAND_RF: u16 = 4;
const PIT_COMMAND_LR: u16 = 5;
const PIT_COMMAND_RR: u16 = 6;
const PIT_COMMAND_CLEAR_TIRES: u16 = 7;
const PIT_COMMAND_FR: u16 = 8;
const PIT_COMMAND_CLEAR_WS: u16 = 9;
const PIT_COMMAND_CLEAR_FR: u16 = 10;
const PIT_COMMAND_CLEAR_FUEL: u16 = 11;

/// Clear all pit checkboxes.
pub fn clear() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_CLEAR, 0))
}

/// Clean the winshield, using one tear off.
pub fn ws() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_WS, 0))
}

/// Add fuel, optionally specify the amount to add in liters or pass '0' to use existing amount.
pub fn fuel(liters: i32) {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_FUEL, liters))
}

/// Change the left front tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure.
pub fn lf(pressure: i32) {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_LF, pressure))
}

/// Change the right front tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure.
pub fn rf(pressure: i32) {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_RF, pressure))
}

/// Change the left rear tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure.
pub fn lr(pressure: i32) {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_LR, pressure))
}

/// Change the right rear tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure.
pub fn rr(pressure: i32) {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_RR, pressure))
}

/// Clear tire pit checkboxes.
pub fn clear_tires() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_CLEAR_TIRES, 0))
}

/// Request a fast repair.
pub fn fr() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_FR, 0))
}

/// Uncheck Clean the winshield checkbox.
pub fn clear_ws() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_CLEAR_WS, 0))
}

/// Uncheck request a fast repair.
pub fn clear_fr() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_CLEAR_FR, 0))
}

/// Uncheck add fuel.
pub fn clear_fuel() {
    BROADCAST_PIT_COMMAND.run((PIT_COMMAND_CLEAR_FUEL, 0))
}
