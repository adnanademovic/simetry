use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Penalty {
    None,
    DriveThroughCutting,
    StopAndGo10Cutting,
    StopAndGo20Cutting,
    StopAndGo30Cutting,
    DisqualifiedCutting,
    RemoveBestLaptimeCutting,
    DriveThroughPitSpeeding,
    StopAndGo10PitSpeeding,
    StopAndGo20PitSpeeding,
    StopAndGo30PitSpeeding,
    DisqualifiedPitSpeeding,
    RemoveBestLaptimePitSpeeding,
    DisqualifiedIgnoredMandatoryPit,
    PostRaceTime,
    DisqualifiedTrolling,
    DisqualifiedPitEntry,
    DisqualifiedPitExit,
    DisqualifiedWrongWay,
    DriveThroughIgnoredDriverStint,
    DisqualifiedIgnoredDriverStint,
    DisqualifiedExceededDriverStintLimit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Off,
    Replay,
    Live,
    Pause,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionType {
    Unknown,
    Practice,
    Qualify,
    Race,
    Hotlap,
    TimeAttack,
    Drift,
    Drag,
    HotStint,
    HotlapSuperPole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlagType {
    None,
    Blue,
    Yellow,
    Black,
    White,
    Checkered,
    Penalty,
    Green,
    Orange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackGripStatus {
    Green,
    Fast,
    Optimum,
    Greasy,
    Damp,
    Wet,
    Flooded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RainIntensity {
    NoRain,
    Drizzle,
    LightRain,
    MediumRain,
    HeavyRain,
    Thunderstorm,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wheels<T> {
    pub front_left: T,
    pub front_right: T,
    pub rear_left: T,
    pub rear_right: T,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CarDamage {
    pub front: f32,
    pub rear: f32,
    pub left: f32,
    pub right: f32,
    pub center: f32,
}

/// Aids that have been currently enabled
#[derive(Clone, Debug, PartialEq)]
pub struct Aids {
    /// Fuel consumption rate
    pub fuel_rate: f32,
    /// Tyre wear rate
    pub tyre_rate: f32,
    /// Mechanical damage rate
    pub mechanical_damage: f32,
    /// Not allowed in Blancpain endurance series
    pub allow_tyre_blankets: f32,
    /// Stability control used
    pub stability: f32,
    /// Auto clutch used
    pub auto_clutch: bool,
    /// Always true in ACC
    pub auto_blip: bool,
}

/// Global flags that are being waved
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalFlags {
    pub yellow: bool,
    pub yellow1: bool,
    pub yellow2: bool,
    pub yellow3: bool,
    pub white: bool,
    pub green: bool,
    pub chequered: bool,
    pub red: bool,
}

/// Data selected on the pitstop mfd
#[derive(Clone, Debug, PartialEq)]
pub struct MfdPitstop {
    pub tyre_set: i32,
    pub fuel_to_add: f32,
    pub tyre_pressures: Wheels<f32>,
}

/// Information about a time in text and in millis
#[derive(Clone, Debug, PartialEq)]
pub struct Time {
    /// Integer in milliseconds
    pub millis: i32,
    /// In text
    pub text: String,
}

/// Information about the state of a single wheel
#[derive(Clone, Debug, PartialEq)]
pub struct WheelInfo {
    /// Tyre pressure
    pub tyre_pressure: f32,
    /// Wheel angular speed in rad/s
    pub angular_speed: f32,
    /// Suspension travel
    pub suspension_travel: f32,
    /// Tyre rubber code temperature
    pub tyre_core_temperature: f32,
    /// Brake disc temperatures
    pub brake_temperature: f32,
    /// Tyre contact point global coordinates
    pub tyre_contact_point: Vector3<f32>,
    /// Tyre contact normal
    pub tyre_contact_normal: Vector3<f32>,
    /// Tyre contact heading
    pub tyre_contact_heading: Vector3<f32>,
    /// Tyre slip
    pub slip: f32,
    /// Tyre slip ratio in radians
    pub slip_ratio: f32,
    /// Tyre slip angle
    pub slip_angle: f32,
    /// Brake pressure
    pub brake_pressure: f32,
    /// Brake pad wear
    pub pad_life: f32,
    /// Brake disk wear
    pub disc_life: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Physics {
    /// Current step index
    pub packet_id: i32,
    /// Gas pedal input value (from -0 to 1.0)
    pub gas: f32,
    /// Brake pedal input value (from -0 to 1.0)
    pub brake: f32,
    /// Clutch pedal input value (from -0 to 1.0)
    pub clutch: f32,
    /// Steering input value (from -1.0 to 1.0)
    pub steer_angle: f32,
    /// Current gear
    pub gear: i32,
    /// Engine revolutions per minute
    pub rpm: i32,
    /// Amount of fuel remaining in kg
    pub fuel: f32,
    /// Car speed in km/h
    pub speed_kmh: f32,
    /// Car velocity vector in global coordinates
    pub velocity: Vector3<f32>,
    /// Car acceleration vector in global coordinates
    pub acc_g: Vector3<f32>,
    /// Information about the wheels
    pub wheels: Wheels<WheelInfo>,
    /// TC in action
    pub tc: f32,
    /// ABS in action
    pub abs: f32,
    /// Car yaw orientation
    pub heading: f32,
    /// Car pitch orientation
    pub pitch: f32,
    /// Car roll orientation
    pub roll: f32,
    /// Car damage in each area
    pub car_damage: CarDamage,
    /// Pit limiter is on
    pub pit_limiter_on: bool,
    /// Automatic transmission is on
    pub auto_shifter_on: bool,
    /// Car turbo level
    pub turbo_boost: f32,
    /// Air temperature
    pub air_temperature: f32,
    /// Road temperature
    pub road_temperature: f32,
    /// Car angular velocity vector in local coordinates
    pub local_angular_velocity: Vector3<f32>,
    /// Force feedback signal
    pub final_ff: f32,
    /// Car is controlled by the AI
    pub is_ai_controlled: bool,
    /// Front brake bias
    pub brake_bias: f32,
    /// Car velocity vector in local coordinates
    pub local_velocity: Vector3<f32>,
    /// Water tempearture
    pub water_temperature: f32,
    /// Brake pad compound front
    pub front_brake_compound: i32,
    /// Brake pad compound rear
    pub rear_brake_compound: i32,
    /// Is ignition switch set to on
    pub ignition_on: bool,
    /// Is starter switch set to on
    pub starter_engine_on: bool,
    /// Is engine running
    pub engine_running: bool,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub kerb_vibration: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub slip_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub g_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub abs_vibrations: f32,
}

/// Lap timing information
#[derive(Clone, Debug, PartialEq)]
pub struct LapTiming {
    /// Current lap time
    pub current: Time,
    /// Last lap time
    pub last: Time,
    /// Best lap time
    pub best: Time,
    /// Last split time
    pub split: Time,
    /// Delta time time
    pub delta_lap: Time,
    /// Estimated lap time
    pub estimated_lap: Time,
    /// Last sector time in milliseconds
    pub last_sector_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Graphics {
    /// Current step index
    pub packet_id: i32,
    pub status: Status,
    pub session: SessionType,
    /// Lap timing information
    pub lap_timing: LapTiming,
    /// Number of completed laps
    pub completed_laps: i32,
    /// Current player position
    pub position: i32,
    /// Session time left
    pub session_time_left: f32,
    /// Distance travelled in the current stint
    pub distance_traveled: f32,
    /// Car is pitting
    pub is_in_pit: bool,
    /// Current track sector
    pub current_sector_index: i32,
    /// Tyre compound used
    pub tyre_compound: String,
    /// Car position on track spline (0.0 start to 1.0)
    pub normalized_car_position: f32,
    /// Positions of cars on track, indexed by their IDs
    pub car_coordinates: BTreeMap<i32, Vector3<f32>>,
    /// Player Car ID
    pub player_car_id: i32,
    /// Penalty time to wait
    pub penalty_time: f32,
    pub flag: FlagType,
    pub penalty: Penalty,
    /// Ideal line is on
    pub ideal_line_on: bool,
    /// Car is in pit lane
    pub is_in_pit_lane: bool,
    /// Mandatory pit is completed
    pub mandatory_pit_done: bool,
    /// Wind speed in m/s
    pub wind_speed: f32,
    /// Wind direction in radians
    pub wind_direction: f32,
    /// Car is working on setup
    pub is_setup_menu_visible: bool,
    /// Current car main display index
    pub main_display_index: i32,
    /// Current car secondary display index
    pub secondary_display_index: i32,
    /// Traction control level
    pub tc: i32,
    /// Traction control cut level
    pub tc_cut: i32,
    /// Current engine map
    pub engine_map: i32,
    /// ABS level
    pub abs: i32,
    /// Average fuel consumed per lap in liters
    pub fuel_used_per_lap: f32,
    /// Rain lights on
    pub rain_lights: bool,
    /// Flashing lights on
    pub flashing_lights: bool,
    /// Current lights stage
    pub lights_stage: i32,
    /// Exhaust temperature
    pub exhaust_temperature: f32,
    /// Current wiper stage
    pub wiper_stage: i32,
    /// Time the driver is allowed to drive in race (ms)
    pub driver_stint_total_time_left: i32,
    /// Time the driver is allowed to drive in stint (ms)
    pub driver_stint_time_left: i32,
    /// Are rain tyres equipped
    pub rain_tyres: bool,
    pub session_index: i32,
    /// Used fuel since last time refueling
    pub used_fuel: f32,
    /// Is delta positive
    pub is_delta_positive: bool,
    /// Is lap valid for timing
    pub is_valid_lap: bool,
    /// Estimated laps possible with current fuel level
    pub fuel_estimated_laps: f32,
    /// Track status (Green, Fast, Optimum, Damp, Wet)
    pub track_status: String,
    /// Mandatory pitstops the player still has to do
    pub missing_mandatory_pits: i32,
    /// Time of day in seconds
    pub clock: f32,
    /// Is left blinker on
    pub direction_lights_left: bool,
    /// Is right blinker on
    pub direction_lights_right: bool,
    /// Global flags that are being waved
    pub global_flags: GlobalFlags,
    /// Data selected on the pitstop mfd
    pub mfd_pitstop: MfdPitstop,
    pub track_grip_status: TrackGripStatus,
    pub rain_intensity: RainIntensity,
    pub rain_intensity_in_10m: RainIntensity,
    pub rain_intensity_in_30m: RainIntensity,
    pub current_tyre_set: i32,
    pub strategy_tyre_set: i32,
}

/// Data that never changes during a session
#[derive(Clone, Debug, PartialEq)]
pub struct StaticData {
    /// Shared memory version
    pub sm_version: String,
    /// Assetto Corsa version
    pub ac_version: String,
    /// Number of sessions
    pub number_of_sessions: i32,
    /// Number of cars
    pub num_cars: i32,
    /// Player car model
    pub car_model: String,
    /// Track name
    pub track: String,
    /// Track configuration
    pub track_configuration: String,
    /// Player name
    pub player_name: String,
    /// Player surname
    pub player_surname: String,
    /// Player nickname
    pub player_nick: String,
    /// Number of sectors
    pub sector_count: i32,
    /// Maximum rpm
    pub max_rpm: i32,
    /// Maximum fuel tank capacity
    pub max_fuel: f32,
    /// Penalties enabled
    pub penalties_enabled: i32,
    /// Aids that have been currently enabled
    pub aids: Aids,
    /// Pit window opening time
    pub pit_window_start: i32,
    /// Pit windows closing time
    pub pit_window_end: i32,
    /// Is it a multiplayer session
    pub is_online: bool,
    /// Name of the dry tyres
    pub dry_tyres_name: String,
    /// Name of the wet tyres
    pub wet_tyres_name: String,
}
