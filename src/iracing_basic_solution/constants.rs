use std::time::Duration;

pub(super) static DATAVALIDEVENTNAME: &[u8] = b"Local\\IRSDKDataValidEvent";
pub(super) static MEMMAPFILENAME: &[u8] = b"Local\\IRSDKMemMapFileName";
pub(super) static BROADCASTMSGNAME: &[u8] = b"IRSDK_BROADCASTMSG";

pub(super) const MAX_BUFS: usize = 4;
pub(super) const MAX_STRING: usize = 32;
// descriptions can be longer than max_string!
pub(super) const MAX_DESC: usize = 64;

/// Value for session lap limit if it's unlimited
pub const UNLIMITED_LAPS: i32 = 32_767;
/// Value for session time limit if it's unlimited
pub const UNLIMITED_TIME: f64 = 604_800.0;

/// Version of telemetry headers
pub const VER: i32 = 2;

pub(super) const STATUS_CONNECTED: i32 = 1;

pub(super) const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
