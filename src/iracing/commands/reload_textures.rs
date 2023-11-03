//! Commands to reload textures.

use super::BROADCAST_RELOAD_TEXTURES;

const RELOAD_TEXTURES_ALL: u16 = 0;
const RELOAD_TEXTURES_CAR_IDX: u16 = 1;

/// Reload textures on all cars.
pub fn all() {
    BROADCAST_RELOAD_TEXTURES.run((RELOAD_TEXTURES_ALL, 0))
}

/// Reload only textures for the specific carIdx.
pub fn car(car_idx: u16) {
    BROADCAST_RELOAD_TEXTURES.run((RELOAD_TEXTURES_CAR_IDX, car_idx))
}
