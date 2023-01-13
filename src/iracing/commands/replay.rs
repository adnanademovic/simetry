//! Commands to control replays.

use super::{
    BROADCAST_REPLAY_SEARCH, BROADCAST_REPLAY_SEARCH_SESSION_TIME,
    BROADCAST_REPLAY_SET_PLAY_POSITION, BROADCAST_REPLAY_SET_PLAY_SPEED,
    BROADCAST_REPLAY_SET_STATE,
};

// clear any data in the replay tape
const RPY_STATE_ERASE_TAPE: u16 = 0;

const RPY_SEARCH_TO_START: u16 = 0;
const RPY_SEARCH_TO_END: u16 = 1;
const RPY_SEARCH_PREV_SESSION: u16 = 2;
const RPY_SEARCH_NEXT_SESSION: u16 = 3;
const RPY_SEARCH_PREV_LAP: u16 = 4;
const RPY_SEARCH_NEXT_LAP: u16 = 5;
const RPY_SEARCH_PREV_FRAME: u16 = 6;
const RPY_SEARCH_NEXT_FRAME: u16 = 7;
const RPY_SEARCH_PREV_INCIDENT: u16 = 8;
const RPY_SEARCH_NEXT_INCIDENT: u16 = 9;

const RPY_POS_BEGIN: u16 = 0;
const RPY_POS_CURRENT: u16 = 1;
const RPY_POS_END: u16 = 2;

/// Play replay at normal speed.
pub fn play() {
    play_with_speed(1, false)
}

/// Rewind replay at normal speed.
pub fn rewind() {
    play_with_speed(-1, false)
}

/// Pause replay.
pub fn pause() {
    play_with_speed(0, false);
}

/// Fast forward replay at given speed, 1-16.
pub fn fast_forward(speed: u8) {
    play_with_speed(speed as i8, false)
}

/// Fast rewind replay at given speed, 1-16.
pub fn fast_rewind(speed: u8) {
    play_with_speed(-(speed as i8), false)
}

/// Play replay slowly at given speed, 1-16.
pub fn slow_forward(speed: u8) {
    play_with_speed(speed as i8, true)
}

/// Rewind replay slowly at given speed, 1-16.
pub fn slow_rewind(speed: u8) {
    play_with_speed(-(speed as i8), true)
}

/// Generic command for controlling play speed.
///
/// A speed of zero represents pause.
///
/// If slow motion is false, the speed of 1-16 represents a multiplier. Otherwise, it's a divider.
pub fn play_with_speed(speed: i8, slow_motion: bool) {
    BROADCAST_REPLAY_SET_PLAY_SPEED.run((speed as u16, slow_motion as u16))
}

/// Search for a given time in milliseconds in the requested session.
pub fn search_session_time(session: u16, millis: i32) {
    BROADCAST_REPLAY_SEARCH_SESSION_TIME.run((session, millis))
}

/// Clear replay tape.
pub fn clear_replay_tape() {
    BROADCAST_REPLAY_SET_STATE.run(RPY_STATE_ERASE_TAPE)
}

pub enum PlayPosition {
    Begin = RPY_POS_BEGIN as isize,
    Current = RPY_POS_CURRENT as isize,
    End = RPY_POS_END as isize,
}

pub fn set_play_position(from: PlayPosition, frames_offset: i32) {
    BROADCAST_REPLAY_SET_PLAY_POSITION.run((from as u16, frames_offset))
}

pub fn search_to_start() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_TO_START)
}

pub fn search_to_end() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_TO_END)
}

pub fn search_prev_session() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_PREV_SESSION)
}

pub fn search_next_session() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_NEXT_SESSION)
}

pub fn search_prev_lap() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_PREV_LAP)
}

pub fn search_next_lap() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_NEXT_LAP)
}

pub fn search_prev_frame() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_PREV_FRAME)
}

pub fn search_next_frame() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_NEXT_FRAME)
}

pub fn search_prev_incident() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_PREV_INCIDENT)
}

pub fn search_next_incident() {
    BROADCAST_REPLAY_SEARCH.run(RPY_SEARCH_NEXT_INCIDENT)
}
