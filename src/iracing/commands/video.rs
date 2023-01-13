//! Commands to control video recording.

use super::BROADCAST_VIDEO_CAPTURE;

const VIDEO_CAPTURE_TRIGGER_SCREEN_SHOT: u16 = 0;
const VIDEO_CAPTURE_START_VIDEO_CAPTURE: u16 = 1;
const VIDEO_CAPTURE_END_VIDEO_CAPTURE: u16 = 2;
const VIDEO_CAPTURE_TOGGLE_VIDEO_CAPTURE: u16 = 3;
const VIDEO_CAPTURE_SHOW_VIDEO_TIMER: u16 = 4;
const VIDEO_CAPTURE_HIDE_VIDEO_TIMER: u16 = 5;

/// Save a screenshot to disk.
pub fn trigger_screen_shot() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_TRIGGER_SCREEN_SHOT)
}

/// Start capturing video.
pub fn start_video_capture() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_START_VIDEO_CAPTURE)
}

/// Stop capturing video.
pub fn end_video_capture() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_END_VIDEO_CAPTURE)
}

/// Toggle video capture on/off.
pub fn toggle_video_capture() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_TOGGLE_VIDEO_CAPTURE)
}

/// Show video timer in upper left corner of display.
pub fn show_video_timer() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_SHOW_VIDEO_TIMER)
}

/// Hide video timer.
pub fn hide_video_timer() {
    BROADCAST_VIDEO_CAPTURE.run(VIDEO_CAPTURE_HIDE_VIDEO_TIMER)
}
