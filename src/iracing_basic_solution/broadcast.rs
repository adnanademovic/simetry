use super::constants::BROADCASTMSGNAME;
use once_cell::sync::Lazy;
use std::ffi::CString;
use winapi::shared::minwindef::{MAKELONG, UINT};
use winapi::um::winuser::{RegisterWindowMessageA, SendNotifyMessageA, HWND_BROADCAST};

static IRACING_BROADCAST_MSG_ID: Lazy<UINT> = Lazy::new(|| {
    let name = CString::new(BROADCASTMSGNAME).unwrap();
    unsafe { RegisterWindowMessageA(name.as_ptr()) }
});

fn broadcast_message_inner_1(code: u16, var1: u16, var2: isize) {
    let param1 = MAKELONG(code, var1);
    unsafe {
        SendNotifyMessageA(
            HWND_BROADCAST,
            *IRACING_BROADCAST_MSG_ID,
            param1 as usize,
            var2,
        )
    };
}

#[allow(dead_code)]
enum Param2 {
    Isize(isize),
    Uint16(u16, u16),
    Float(f32),
}

impl Param2 {
    fn into_raw(self) -> isize {
        match self {
            Param2::Isize(a) => a,
            Param2::Uint16(a, b) => MAKELONG(a, b) as isize,
            Param2::Float(a) => (a * 65536.0) as isize,
        }
    }
}

pub enum ReloadTexturesMode {
    All,
    CarIdx(u16),
}

pub enum Message {
    /// car position, group, camera
    CamSwitchPos,
    /// driver #, group, camera
    CamSwitchNum,
    /// CameraState, unused, unused
    CamSetState,
    /// speed, slowMotion, unused
    ReplaySetPlaySpeed,
    /// RpyPosMode, Frame Number (high, low)
    ReplaySetPlayPosition,
    /// RpySrchMode, unused, unused
    ReplaySearch,
    /// RpyStateMode, unused, unused
    ReplaySetState,
    /// ReloadTexturesMode, carIdx, unused
    ReloadTextures(ReloadTexturesMode),
    /// ChatCommandMode, subCommand, unused
    ChatComand,
    /// Change elements of the pit strategy
    PitCommand(PitCommand),
    /// TelemCommandMode, unused, unused
    TelemCommand,
    /// Set the max force of the steering wheel
    ///
    /// Sending `Some(value)` sets the max force in Nm, locking the in game options.
    ///
    /// Sending `None` unlocks the in game options
    FFBCommand { max_force: Option<f32> },
    /// sessionNum, sessionTimeMS (high, low)
    ReplaySearchSessionTime,
}

impl Message {
    fn message_code(&self) -> u16 {
        match self {
            Message::CamSwitchPos => 0,
            Message::CamSwitchNum => 1,
            Message::CamSetState => 2,
            Message::ReplaySetPlaySpeed => 3,
            Message::ReplaySetPlayPosition => 4,
            Message::ReplaySearch => 5,
            Message::ReplaySetState => 6,
            Message::ReloadTextures(_) => 7,
            Message::ChatComand => 8,
            Message::PitCommand(_) => 9,
            Message::TelemCommand => 10,
            Message::FFBCommand { .. } => 11,
            Message::ReplaySearchSessionTime => 12,
        }
    }

    pub fn broadcast(&self) {
        let code = self.message_code();
        let (param1, param2) = match self {
            Message::CamSwitchPos => return,
            Message::CamSwitchNum => return,
            Message::CamSetState => return,
            Message::ReplaySetPlaySpeed => return,
            Message::ReplaySetPlayPosition => return,
            Message::ReplaySearch => return,
            Message::ReplaySetState => return,
            Message::ReloadTextures(ReloadTexturesMode::All) => (0, Param2::Isize(0)),
            Message::ReloadTextures(ReloadTexturesMode::CarIdx(car_idx)) => {
                (1, Param2::Uint16(*car_idx, 0))
            }
            Message::ChatComand => return,
            Message::PitCommand(pit_command) => pit_command.broadcast_params(),
            Message::TelemCommand => return,
            Message::FFBCommand { max_force } => (0, Param2::Float(max_force.unwrap_or(-1.0))),
            Message::ReplaySearchSessionTime => return,
        };
        broadcast_message_inner_1(code, param1, param2.into_raw())
    }
}

pub enum PitCommand {
    /// Clear all pit checkboxes
    Clear,
    /// Clean the windshield, using one tear off
    Windshield,
    /// Add fuel
    ///
    /// If you pass `None`, refuels by the amount specified in the black box.
    ///
    /// If you pass a value greater than zero, the refuel amount is set to that in liters.
    Fuel(Option<isize>),
    /// Change the left front tire
    ///
    /// Optionally specify the pressure in KPa or pass `None` to use existing pressure.
    ///
    /// Bear in mind that the amount will be rounded down to fit the discrete steps in sim.
    LeftFront(Option<isize>),
    /// Change the right front tire
    ///
    /// Optionally specify the pressure in KPa or pass `None` to use existing pressure.
    ///
    /// Bear in mind that the amount will be rounded down to fit the discrete steps in sim.
    RightFront(Option<isize>),
    /// Change the left rear tire
    ///
    /// Optionally specify the pressure in KPa or pass `None` to use existing pressure.
    ///
    /// Bear in mind that the amount will be rounded down to fit the discrete steps in sim.
    LeftRear(Option<isize>),
    /// Change the right rear tire
    ///
    /// Optionally specify the pressure in KPa or pass `None` to use existing pressure.
    ///
    /// Bear in mind that the amount will be rounded down to fit the discrete steps in sim.
    RightRear(Option<isize>),
    /// Do not change any tires
    ClearTires,
    /// Request a fast repair
    FastRepair,
    /// Do not clean the windshield checkbox
    ClearWindshield,
    /// Do not request a fast repair
    ClearFastRepair,
    /// Do not add fuel
    ClearFuel,
}

impl PitCommand {
    fn message_code(&self) -> u16 {
        match self {
            PitCommand::Clear => 0,
            PitCommand::Windshield => 1,
            PitCommand::Fuel(_) => 2,
            PitCommand::LeftFront(_) => 3,
            PitCommand::RightFront(_) => 4,
            PitCommand::LeftRear(_) => 5,
            PitCommand::RightRear(_) => 6,
            PitCommand::ClearTires => 7,
            PitCommand::FastRepair => 8,
            PitCommand::ClearWindshield => 9,
            PitCommand::ClearFastRepair => 10,
            PitCommand::ClearFuel => 11,
        }
    }

    fn broadcast_params(&self) -> (u16, Param2) {
        let subcode = self.message_code();
        let params = match self {
            PitCommand::Clear => 0,
            PitCommand::Windshield => 0,
            PitCommand::Fuel(val) => val.unwrap_or(0),
            PitCommand::LeftFront(val) => val.unwrap_or(0),
            PitCommand::RightFront(val) => val.unwrap_or(0),
            PitCommand::LeftRear(val) => val.unwrap_or(0),
            PitCommand::RightRear(val) => val.unwrap_or(0),
            PitCommand::ClearTires => 0,
            PitCommand::FastRepair => 0,
            PitCommand::ClearWindshield => 0,
            PitCommand::ClearFastRepair => 0,
            PitCommand::ClearFuel => 0,
        };
        (subcode, Param2::Isize(params))
    }
}
