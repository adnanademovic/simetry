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

pub mod global_flags {
    pub const CHECKERED: u32 = 0x0000_0001;
    pub const WHITE: u32 = 0x0000_0002;
    pub const GREEN: u32 = 0x0000_0004;
    pub const YELLOW: u32 = 0x0000_0008;
    pub const RED: u32 = 0x0000_0010;
    pub const BLUE: u32 = 0x0000_0020;
    pub const DEBRIS: u32 = 0x0000_0040;
    pub const CROSSED: u32 = 0x0000_0080;
    pub const YELLOW_WAVING: u32 = 0x0000_0100;
    pub const ONE_LAP_TO_GREEN: u32 = 0x0000_0200;
    pub const GREEN_HELD: u32 = 0x0000_0400;
    pub const TEN_TO_GO: u32 = 0x0000_0800;
    pub const FIVE_TO_GO: u32 = 0x0000_1000;
    pub const RANDOM_WAVING: u32 = 0x0000_2000;
    pub const CAUTION: u32 = 0x0000_4000;
    pub const CAUTION_WAVING: u32 = 0x0000_8000;
}

pub mod driver_black_flags {
    pub const BLACK: u32 = 0x0001_0000;
    pub const DISQUALIFY: u32 = 0x0002_0000;
    pub const SERVICEABLE: u32 = 0x0004_0000;
    pub const FURLED: u32 = 0x0008_0000;
    pub const REPAIR: u32 = 0x0010_0000;
}

pub mod start_flags {
    pub const HIDDEN: u32 = 0x1000_0000;
    pub const READY: u32 = 0x2000_0000;
    pub const SET: u32 = 0x4000_0000;
    pub const GO: u32 = 0x8000_0000;
}
