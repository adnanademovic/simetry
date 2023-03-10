use crate::iracing::flags::{driver_black_flags, global_flags, start_flags};
use crate::iracing::{
    BitField, CarPositions, Header, Value, VarData, VarHeader, VarHeaders, VarType,
};
use crate::{BasicTelemetry, MomentImpl, RacingFlags};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::meter_per_second;
use yaml_rust::Yaml;

#[derive(Clone)]
pub struct SimState {
    header: Arc<Header>,
    variables: Arc<VarHeaders>,
    raw_data: Vec<u8>,
    session_info: Arc<Yaml>,
}

impl MomentImpl for SimState {
    fn car_left(&self) -> bool {
        self.read_name("CarLeftRight")
            .unwrap_or(CarPositions::Off)
            .car_left()
    }

    fn car_right(&self) -> bool {
        self.read_name("CarLeftRight")
            .unwrap_or(CarPositions::Off)
            .car_right()
    }

    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        Some(BasicTelemetry {
            gear: self.read_name("Gear").unwrap_or(0i32) as i8,
            speed: Velocity::new::<meter_per_second>(self.read_name("Speed").unwrap_or(0.0)),
            engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.read_name("RPM").unwrap_or(0.0),
            ),
            max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.session_info()["DriverInfo"]["DriverCarRedLine"]
                    .as_f64()
                    .unwrap_or(f64::INFINITY),
            ),
            pit_limiter_engaged: self.read_name("dcPitSpeedLimiterToggle").unwrap_or(false),
            in_pit_lane: self.read_name("OnPitRoad").unwrap_or(false),
        })
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        Some(AngularVelocity::new::<revolution_per_minute>(
            self.session_info()["DriverInfo"]["DriverCarSLShiftRPM"].as_f64()?,
        ))
    }

    fn flags(&self) -> RacingFlags {
        let flags: BitField = self.read_name("SessionFlags").unwrap_or(BitField(0));

        RacingFlags {
            green: flags.0 & global_flags::GREEN != 0,
            yellow: flags.0 & global_flags::YELLOW != 0,
            blue: flags.0 & global_flags::BLUE != 0,
            white: flags.0 & global_flags::WHITE != 0,
            red: flags.0 & global_flags::RED != 0,
            black: flags.0
                & (driver_black_flags::BLACK
                    | driver_black_flags::DISQUALIFY
                    | driver_black_flags::FURLED)
                != 0,
            checkered: flags.0 & global_flags::CHECKERED != 0,
            meatball: flags.0 & (driver_black_flags::REPAIR | driver_black_flags::SERVICEABLE) != 0,
            black_and_white: false,
            start_ready: flags.0 & start_flags::READY != 0,
            start_set: flags.0 & start_flags::SET != 0,
            start_go: flags.0 & start_flags::GO != 0,
        }
    }

    fn car_model_id(&self) -> Option<String> {
        let driver_info = &self.session_info["DriverInfo"];
        let player_car_idx = driver_info["DriverCarIdx"].as_i64()?;
        let player_driver = driver_info["Drivers"]
            .as_vec()?
            .iter()
            .find(|driver| driver["CarIdx"].as_i64() == Some(player_car_idx))?;
        let car_model_id = player_driver["CarID"].as_i64()?;
        Some(format!("{car_model_id}"))
    }

    fn ignition_on(&self) -> bool {
        self.read_name("Voltage").unwrap_or(0.0f32) > 1.0
    }

    fn starter_on(&self) -> bool {
        self.read_name("dcStarter").unwrap_or(false)
    }
}

impl Debug for SimState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimState")
            .field("header", &self.header)
            .field("session_info", &self.session_info)
            .field("data", &DataDebugPrinter(self))
            .finish()
    }
}

impl SimState {
    pub(super) fn new(
        header: Arc<Header>,
        variables: Arc<VarHeaders>,
        raw_data: Vec<u8>,
        session_info: Arc<Yaml>,
    ) -> Self {
        Self {
            header,
            variables,
            raw_data,
            session_info,
        }
    }

    pub fn read<T: VarData>(&self, var: &VarHeader) -> Option<T> {
        self.read_at(0, var)
    }

    pub fn read_at<T: VarData>(&self, idx: usize, var: &VarHeader) -> Option<T> {
        T::parse_from_raw(idx, var, &self.raw_data)
    }

    pub fn read_name<T: VarData>(&self, name: &str) -> Option<T> {
        self.read_name_at(name, 0)
    }

    pub fn read_name_at<T: VarData>(&self, name: &str, idx: usize) -> Option<T> {
        self.read_at(idx, self.variables.get(name)?)
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn variables(&self) -> &VarHeaders {
        &self.variables
    }

    pub fn session_info(&self) -> &Yaml {
        &self.session_info
    }
}

struct DataDebugPrinter<'a>(&'a SimState);

impl<'a> Debug for DataDebugPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;
        f.debug_list()
            .entries(self.0.variables().values().map(|header| {
                Variable {
                    name: &header.name,
                    description: &header.desc,
                    datatype: header.var_type,
                    unit: &header.unit,
                    count_as_time: header.count_as_time,
                    data: (0..header.count)
                        .map(|idx| {
                            format!(
                                "{}",
                                self.0
                                    .read_at::<Value>(idx, header)
                                    .unwrap_or(Value::Char(b'?'))
                            )
                        })
                        .join(", "),
                }
            }))
            .finish()
    }
}

#[derive(Debug)]
struct Variable<'a> {
    #[allow(dead_code)]
    name: &'a str,
    #[allow(dead_code)]
    description: &'a str,
    #[allow(dead_code)]
    datatype: VarType,
    #[allow(dead_code)]
    unit: &'a str,
    #[allow(dead_code)]
    count_as_time: bool,
    #[allow(dead_code)]
    data: String,
}
