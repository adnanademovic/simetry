use once_cell::sync::Lazy;
use std::marker::PhantomData;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{RegisterWindowMessageA, SendNotifyMessageA};

static BROADCASTMSGNAME: &[u8] = b"IRSDK_BROADCASTMSG\0";

static IRACING_BROADCAST_MSG_ID: Lazy<u32> =
    Lazy::new(|| unsafe { RegisterWindowMessageA(PCSTR::from_raw(BROADCASTMSGNAME.as_ptr())) });

struct RawParams {
    var1: u16,
    var2: isize,
}

fn make_long(a: u16, b: u16) -> u32 {
    (a as u32) | ((b as u32) << 16)
}

type Param1 = u16;

type Param2u = (u16, u16);

type Param3 = (u16, u16, u16);

type Param2i = (u16, i32);

type Param2f = (u16, f32);

impl From<Param1> for RawParams {
    fn from(var1: Param1) -> Self {
        Self { var1, var2: 0 }
    }
}

impl From<Param2u> for RawParams {
    fn from((var1, var2): Param2u) -> Self {
        Self {
            var1,
            var2: make_long(var2, 0u16) as isize,
        }
    }
}

impl From<Param3> for RawParams {
    fn from((var1, var2, var3): Param3) -> Self {
        Self {
            var1,
            var2: make_long(var2, var3) as isize,
        }
    }
}

impl From<Param2i> for RawParams {
    fn from((var1, var2): Param2i) -> Self {
        Self {
            var1,
            var2: var2 as isize,
        }
    }
}

impl From<Param2f> for RawParams {
    fn from((var1, var2): Param2f) -> Self {
        Self {
            var1,
            var2: (var2 * 65536.0) as isize,
        }
    }
}

struct Command<T> {
    code: u16,
    phantom: PhantomData<T>,
}

impl<T: Into<RawParams>> Command<T> {
    const fn new(code: u16) -> Self {
        Self {
            code,
            phantom: PhantomData,
        }
    }

    fn run(&self, args: T) {
        let params = args.into();
        let param1 = make_long(self.code, params.var1) as usize;
        unsafe {
            SendNotifyMessageA(
                HWND(0xffff),
                *IRACING_BROADCAST_MSG_ID,
                WPARAM(param1),
                LPARAM(params.var2),
            )
        };
    }
}

const BROADCAST_CAM_SWITCH_POS: Command<Param3> = Command::new(0);
const BROADCAST_CAM_SWITCH_NUM: Command<Param3> = Command::new(1);
const BROADCAST_CAM_SET_STATE: Command<Param1> = Command::new(2);
const BROADCAST_REPLAY_SET_PLAY_SPEED: Command<Param2u> = Command::new(3);
const BROADCAST_REPLAY_SET_PLAY_POSITION: Command<Param2i> = Command::new(4);
const BROADCAST_REPLAY_SEARCH: Command<Param1> = Command::new(5);
const BROADCAST_REPLAY_SET_STATE: Command<Param1> = Command::new(6);
const BROADCAST_RELOAD_TEXTURES: Command<Param2u> = Command::new(7);
const BROADCAST_CHAT_COMMAND: Command<Param2u> = Command::new(8);
const BROADCAST_PIT_COMMAND: Command<Param2i> = Command::new(9);
const BROADCAST_TELEM_COMMAND: Command<Param1> = Command::new(10);
const BROADCAST_FFBCOMMAND: Command<Param2f> = Command::new(11);
const BROADCAST_REPLAY_SEARCH_SESSION_TIME: Command<Param2i> = Command::new(12);
const BROADCAST_VIDEO_CAPTURE: Command<Param1> = Command::new(13);

pub mod camera;
pub mod chat;
pub mod force_feedback;
pub mod pit;
pub mod reload_textures;
pub mod replay;
pub mod telemetry;
pub mod video;
