use bitmask::bitmask;

bitmask! {
    /// Description of camera state.
    pub mask CameraState: u16 where
    /// States that are part of the camera.
    flags CameraFlag {
        IsSessionScreen       = 0x0001,
        IsScenicActive        = 0x0002,
        /// Can be set in [`commands::camera::set_state`].
        CamToolActive         = 0x0004,
        /// Can be set in [`commands::camera::set_state`].
        UIHidden              = 0x0008,
        /// Can be set in [`commands::camera::set_state`].
        UseAutoShotSelection  = 0x0010,
        /// Can be set in [`commands::camera::set_state`].
        UseTemporaryEdits     = 0x0020,
        /// Can be set in [`commands::camera::set_state`].
        UseKeyAcceleration    = 0x0040,
        /// Can be set in [`commands::camera::set_state`].
        UseKey10xAcceleration = 0x0080,
        /// Can be set in [`commands::camera::set_state`].
        UseMouseAimMode       = 0x0100,
    }
}
