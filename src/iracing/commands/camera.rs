//! Commands to control camera.

use super::{BROADCAST_CAM_SET_STATE, BROADCAST_CAM_SWITCH_NUM, BROADCAST_CAM_SWITCH_POS};
use crate::iracing::CameraState;

/// Switch to a car position and camera group.
///
/// Setting the car position to zero just changes group.
pub fn switch_to_position(car_position: u16, group: u16, camera: u16) {
    BROADCAST_CAM_SWITCH_POS.run((car_position, group, camera))
}

/// Switch to a car number and camera group.
pub fn switch_to_car_number(car_number: u16, group: u16, camera: u16) {
    BROADCAST_CAM_SWITCH_NUM.run((car_number, group, camera))
}

/// Sets the state of the camera.
pub fn set_state(camera_state: CameraState) {
    BROADCAST_CAM_SET_STATE.run(*camera_state);
}
