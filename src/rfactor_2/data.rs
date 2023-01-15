use crate::rfactor_2::shared_memory_data::{
    PageExtended, PageForceFeedback, PageHeader, PageMultiRules, PagePhysicsOptions, PagePitInfo,
    PageRules, PageScoring, PageScoringInfo, PageSessionTransitionCapture, PageTelemetry,
    PageTrackRules, PageTrackRulesAction, PageTrackRulesParticipant, PageTrackedDamage, PageVec3,
    PageVehScoringCapture, PageVehicleScoring, PageVehicleTelemetry, PageWeather,
    PageWheelTelemetry, MAX_MAPPED_IDS, MAX_MAPPED_VEHICLES,
};
use crate::windows_util::cp1252_to_string;
use anyhow::{bail, Error, Result};

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct PacketId(pub u32);

#[derive(Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct ForceFeedback {
    /// Current FFB value
    pub force_value: f64,
}

#[derive(Clone, Debug)]
pub struct Telemetry {
    pub packet_id: PacketId,
    pub vehicles: Vec<VehicleTelemetry>,
}

#[derive(Clone, Debug)]
pub struct VehicleTelemetry {
    // Time
    /// slot ID (note that it can be re-used in multiplayer after someone leaves)    
    pub id: i32,
    /// time since last update (seconds)    
    pub delta_time: f64,
    /// game session time    
    pub elapsed_time: f64,
    /// current lap number    
    pub lap_number: i32,
    /// time this lap was started    
    pub lap_start_et: f64,
    /// current vehicle name    
    pub vehicle_name: String,
    /// current track name    
    pub track_name: String,

    // Position and derivatives
    /// world position in meters    
    pub pos: Vec3,
    /// velocity (meters/sec) in local vehicle coordinates    
    pub local_vel: Vec3,
    /// acceleration (meters/sec^2) in local vehicle coordinates    
    pub local_accel: Vec3,

    // Orientation and derivatives
    /// rows of orientation matrix (use TelemQuat conversions if desired), also converts local    
    pub ori: [Vec3; 3],
    // vehicle vectors into world X, Y, or Z using dot product of rows 0, 1, or 2 respectively
    /// rotation (radians/sec) in local vehicle coordinates    
    pub local_rot: Vec3,
    /// rotational acceleration (radians/sec^2) in local vehicle coordinates    
    pub local_rot_accel: Vec3,

    // Vehicle status
    /// -1=reverse, 0=neutral, 1+=forward gears    
    pub gear: i32,
    /// engine RPM    
    pub engine_rpm: f64,
    /// Celsius    
    pub engine_water_temp: f64,
    /// Celsius    
    pub engine_oil_temp: f64,
    /// clutch RPM    
    pub clutch_rpm: f64,

    // Driver input
    /// ranges  0.0-1.0    
    pub unfiltered_throttle: f64,
    /// ranges  0.0-1.0    
    pub unfiltered_brake: f64,
    /// ranges -1.0-1.0 (left to right)    
    pub unfiltered_steering: f64,
    /// ranges  0.0-1.0    
    pub unfiltered_clutch: f64,

    // Filtered input (various adjustments for rev or speed limiting, TC, ABS?, speed sensitive steering, clutch work for semi-automatic shifting, etc.)
    /// ranges  0.0-1.0    
    pub filtered_throttle: f64,
    /// ranges  0.0-1.0    
    pub filtered_brake: f64,
    /// ranges -1.0-1.0 (left to right)    
    pub filtered_steering: f64,
    /// ranges  0.0-1.0    
    pub filtered_clutch: f64,

    // Misc
    /// torque around steering shaft (used to be mSteeringArmForce, but that is not necessarily accurate for feedback purposes)    
    pub steering_shaft_torque: f64,
    /// deflection at front 3rd spring    
    pub front3rd_deflection: f64,
    /// deflection at rear 3rd spring    
    pub rear3rd_deflection: f64,

    // Aerodynamics
    /// front wing height    
    pub front_wing_height: f64,
    /// front ride height    
    pub front_ride_height: f64,
    /// rear ride height    
    pub rear_ride_height: f64,
    /// drag    
    pub drag: f64,
    /// front downforce    
    pub front_downforce: f64,
    /// rear downforce    
    pub rear_downforce: f64,

    // State/damage info
    /// amount of fuel (liters)    
    pub fuel: f64,
    /// rev limit    
    pub engine_max_rpm: f64,
    /// number of scheduled pitstops    
    pub scheduled_stops: u8,
    /// whether overheating icon is shown    
    pub overheating: u8,
    /// whether any parts (besides wheels) have been detached    
    pub detached: u8,
    /// whether headlights are on    
    pub headlights: u8,
    /// dent severity at 8 locations around the car (0=none, 1=some, 2=more)    
    pub dent_severity: [u8; 8],
    /// time of last impact    
    pub last_impact_et: f64,
    /// magnitude of last impact    
    pub last_impact_magnitude: f64,
    /// location of last impact    
    pub last_impact_pos: Vec3,

    // Expanded
    /// current engine torque (including additive torque) (used to be mEngineTq, but there's little reason to abbreviate it)    
    pub engine_torque: f64,
    /// the current sector (zero-based) with the pitlane stored in the sign bit (example: entering pits from third sector gives 0x80000002)    
    pub current_sector: i32,
    /// whether speed limiter is on    
    pub speed_limiter: u8,
    /// maximum forward gears    
    pub max_gears: u8,
    /// index within brand    
    pub front_tire_compound_index: u8,
    /// index within brand    
    pub rear_tire_compound_index: u8,
    /// capacity in liters    
    pub fuel_capacity: f64,
    /// whether front flap is activated    
    pub front_flap_activated: u8,
    /// whether rear flap is activated    
    pub rear_flap_activated: u8,
    pub rear_flap_legal_status: u8,
    pub ignition_starter: u8,

    /// name of front tire compound    
    pub front_tire_compound_name: String,
    /// name of rear tire compound    
    pub rear_tire_compound_name: String,

    /// whether speed limiter is available    
    pub speed_limiter_available: u8,
    /// whether (hard) anti-stall is activated    
    pub anti_stall_activated: u8,

    /// the *visual* steering wheel range    
    pub visual_steering_wheel_range: f32,

    /// fraction of brakes on rear    
    pub rear_brake_bias: f64,
    /// current turbo boost pressure if available    
    pub turbo_boost_pressure: f64,
    /// offset from static CG to graphical center    
    pub physics_to_graphics_offset: [f32; 3],
    /// the *physical* steering wheel range    
    pub physical_steering_wheel_range: f32,

    // keeping this at the end of the structure to make it easier to replace in future versions
    /// wheel info (front left, front right, rear left, rear right)    
    pub wheels: [WheelTelemetry; 4],
}

#[derive(Clone, Debug)]
pub struct WheelTelemetry {
    /// meters
    pub suspension_deflection: f64,
    /// meters
    pub ride_height: f64,
    /// pushrod load in Newtons
    pub susp_force: f64,
    /// Celsius
    pub brake_temp: f64,
    /// currently 0.0-1.0, depending on driver input and brake balance; will convert to true brake pressure (kPa) in future
    pub brake_pressure: f64,

    /// radians/sec
    pub rotation: f64,
    /// lateral velocity at contact patch
    pub lateral_patch_vel: f64,
    /// longitudinal velocity at contact patch
    pub longitudinal_patch_vel: f64,
    /// lateral velocity at contact patch
    pub lateral_ground_vel: f64,
    /// longitudinal velocity at contact patch
    pub longitudinal_ground_vel: f64,
    /// radians (positive is left for left-side wheels, right for right-side wheels)
    pub camber: f64,
    /// Newtons
    pub lateral_force: f64,
    /// Newtons
    pub longitudinal_force: f64,
    /// Newtons
    pub tire_load: f64,

    /// an approximation of what fraction of the contact patch is sliding
    pub grip_fract: f64,
    /// kPa (tire pressure)
    pub pressure: f64,
    /// Kelvin (subtract 273.15 to get Celsius), left/center/right (not to be confused with inside/center/outside!)
    pub temperature: [f64; 3],
    /// wear (0.0-1.0, fraction of maximum) ... this is not necessarily proportional with grip loss
    pub wear: f64,
    /// the material prefixes from the TDF file
    pub terrain_name: String,
    /// Enum for surface type
    pub surface_type: u8,
    /// whether tire is flat
    pub flat: u8,
    /// whether wheel is detached
    pub detached: u8,
    /// tire radius in centimeters
    pub static_undeflected_radius: u8,

    /// how much is tire deflected from its (speed-sensitive) radius
    pub vertical_tire_deflection: f64,
    /// wheel's y location relative to vehicle y location
    pub wheel_ylocation: f64,
    /// current toe angle w.r.t. the vehicle
    pub toe: f64,

    /// rough average of temperature samples from carcass (Kelvin)
    pub tire_carcass_temperature: f64,
    /// rough average of temperature samples from innermost layer of rubber (before carcass) (Kelvin)
    pub tire_inner_layer_temperature: [f64; 3],
}

#[derive(Clone, Debug)]
pub struct Scoring {
    pub packet_id: PacketId,

    pub scoring_info: ScoringInfo,

    pub vehicles: Vec<VehicleScoring>,
}

#[derive(Clone, Debug)]
pub struct ScoringInfo {
    /// current track name
    pub track_name: String,
    /// current session (0=testday 1-4=practice 5-8=qual 9=warmup 10-13=race)
    pub session: i32,
    /// current time
    pub current_et: f64,
    /// ending time
    pub end_et: f64,
    /// maximum laps
    pub max_laps: i32,
    /// distance around track
    pub lap_dist: f64,

    /// Game phases
    pub game_phase: u8,

    /// Yellow flag states (applies to full-course only)
    pub yellow_flag_state: i8,

    /// whether there are any local yellows at the moment in each sector (not sure if sector 0 is first or last, so test)
    pub sector_flag: [i8; 3],
    /// start light frame (number depends on track)
    pub start_light: u8,
    /// number of red lights in start sequence
    pub num_red_lights: u8,
    /// in realtime as opposed to at the monitor
    pub in_realtime: u8,
    /// player name (including possible multiplayer override)
    pub player_name: String,
    /// may be encoded to be a legal filename
    pub plr_file_name: String,

    // weather
    /// cloud darkness? 0.0-1.0
    pub dark_cloud: f64,
    /// raining severity 0.0-1.0
    pub raining: f64,
    /// temperature (Celsius)
    pub ambient_temp: f64,
    /// temperature (Celsius)
    pub track_temp: f64,
    /// wind speed
    pub wind: Vec3,
    /// minimum wetness on main path 0.0-1.0
    pub min_path_wetness: f64,
    /// maximum wetness on main path 0.0-1.0
    pub max_path_wetness: f64,

    // multiplayer
    /// 1 = server, 2 = client, 3 = server and client
    pub game_mode: u8,
    /// is the server password protected
    pub is_password_protected: u8,
    /// the port of the server (if on a server)
    pub server_port: u16,
    /// the public IP address of the server (if on a server)
    pub server_public_ip: u32,
    /// maximum number of vehicles that can be in the session
    pub max_players: i32,
    /// name of the server
    pub server_name: String,
    /// start time (seconds since midnight) of the event
    pub start_et: f32,

    /// average wetness on main path 0.0-1.0
    pub avg_path_wetness: f64,
}

#[derive(Clone, Debug)]
pub struct VehicleScoring {
    /// slot ID (note that it can be re-used in multiplayer after someone leaves)
    pub id: i32,
    /// driver name
    pub driver_name: String,
    /// vehicle name
    pub vehicle_name: String,
    /// laps completed
    pub total_laps: i16,
    /// Sector
    pub sector: i8,
    /// Finish status
    pub finish_status: i8,
    /// current distance around track
    pub lap_dist: f64,
    /// lateral position with respect to *very approximate* "center" path
    pub path_lateral: f64,
    /// track edge (w.r.t. "center" path) on same side of track as vehicle
    pub track_edge: f64,

    /// best sector 1
    pub best_sector1: f64,
    /// best sector 2 (plus sector 1)
    pub best_sector2: f64,
    /// best lap time
    pub best_lap_time: f64,
    /// last sector 1
    pub last_sector1: f64,
    /// last sector 2 (plus sector 1)
    pub last_sector2: f64,
    /// last lap time
    pub last_lap_time: f64,
    /// current sector 1 if valid
    pub cur_sector1: f64,
    /// current sector 2 (plus sector 1) if valid
    pub cur_sector2: f64,
    // no current laptime because it instantly becomes "last"
    /// number of pitstops made    
    pub num_pitstops: i16,
    /// number of outstanding penalties    
    pub num_penalties: i16,
    /// is this the player's vehicle    
    pub is_player: u8,

    /// Who is in control    
    pub control: i8,
    /// between pit entrance and pit exit (not always accurate for remote vehicles)    
    pub in_pits: u8,
    /// 1-based position    
    pub place: u8,
    /// vehicle class    
    pub vehicle_class: String,

    // Dash Indicators
    /// time behind vehicle in next higher place    
    pub time_behind_next: f64,
    /// laps behind vehicle in next higher place    
    pub laps_behind_next: i32,
    /// time behind leader    
    pub time_behind_leader: f64,
    /// laps behind leader    
    pub laps_behind_leader: i32,
    /// time this lap was started    
    pub lap_start_et: f64,

    // Position and derivatives
    /// world position in meters    
    pub pos: Vec3,
    /// velocity (meters/sec) in local vehicle coordinates    
    pub local_vel: Vec3,
    /// acceleration (meters/sec^2) in local vehicle coordinates    
    pub local_accel: Vec3,

    // Orientation and derivatives
    /// rows of orientation matrix (use TelemQuat conversions if desired), also converts local    
    pub ori: [Vec3; 3],
    // vehicle vectors into world X, Y, or Z using dot product of rows 0, 1, or 2 respectively
    /// rotation (radians/sec) in local vehicle coordinates    
    pub local_rot: Vec3,
    /// rotational acceleration (radians/sec^2) in local vehicle coordinates    
    pub local_rot_accel: Vec3,

    // tag.2012.03.01 - stopped casting some of these so variables now have names and mExpansion has shrunk, overall size and old data locations should be same
    /// status of headlights    
    pub headlights: u8,
    pub pit_state: u8,
    /// whether this vehicle is being scored by server (could be off in qualifying or racing heats)    
    pub server_scored: u8,
    /// game phases (described below) plus 9=after formation, 10=under yellow, 11=under blue (not used)    
    pub individual_phase: u8,

    /// 1-based, can be -1 when invalid    
    pub qualification: i32,

    /// estimated time into lap    
    pub time_into_lap: f64,
    /// estimated laptime used for 'time behind' and 'time into lap' (note: this may changed based on vehicle and setup!?)    
    pub estimated_lap_time: f64,

    /// pit group (same as team name unless pit is shared)    
    pub pit_group: String,
    /// primary flag being shown to vehicle    
    pub flag: u8,
    /// whether this car has taken a full-course caution flag at the start/finish line    
    pub under_yellow: u8,
    pub count_lap_flag: u8,
    /// appears to be within the correct garage stall    
    pub in_garage_stall: u8,

    /// Coded upgrades    
    pub upgrade_pack: String,

    /// location of pit in terms of lap distance    
    pub pit_lap_dist: f32,

    /// sector 1 time from best lap (not necessarily the best sector 1 time)    
    pub best_lap_sector1: f32,
    /// sector 2 time from best lap (not necessarily the best sector 2 time)    
    pub best_lap_sector2: f32,
}

#[derive(Clone, Debug)]
pub struct Rules {
    pub packet_id: PacketId,

    pub track_rules: TrackRules,
    pub actions: Vec<TrackRulesAction>,
    pub participants: Vec<TrackRulesParticipant>,
}

#[derive(Clone, Debug)]
pub struct TrackRulesAction {
    // input only
    /// recommended action
    pub command: i32,
    /// slot ID if applicable
    pub id: i32,
    /// elapsed time that event occurred, if applicable
    pub elapsed_time: f64,
}

#[derive(Clone, Debug)]
pub struct TrackRulesParticipant {
    // input only
    /// slot ID
    pub id: i32,
    /// 0-based place when caution came out (not valid for formation laps)
    pub frozen_order: i16,
    /// 1-based place (typically used for the initialization of the formation lap track order)
    pub place: i16,
    /// a rating of how much this vehicle is contributing to a yellow flag (the sum of all vehicles is compared to TrackRulesV01::mSafetyCarThreshold)
    pub yellow_severity: f32,
    /// equal to ( ( ScoringInfoV01::mLapDist * this->mRelativeLaps ) + VehicleScoringInfoV01::mLapDist )
    pub current_relative_distance: f64,

    // input/output
    /// current formation/caution laps relative to safety car (should generally be zero except when safety car crosses s/f line); this can be decremented to implement 'wave around' or 'beneficiary rule' (a.k.a. 'lucky dog' or 'free pass')
    pub relative_laps: i32,
    /// which column (line/lane) that participant is supposed to be in
    pub column_assignment: i32,
    /// 0-based position within column (line/lane) that participant is supposed to be located at (-1 is invalid)
    pub position_assignment: i32,
    /// whether the rules allow this particular vehicle to enter pits right now (input is 2=false or 3=true; if you want to edit it, set to 0=false or 1=true)
    pub pits_open: u8,
    /// while in the frozen order, this flag indicates whether the vehicle can be followed (this should be false for somebody who has temporarily spun and hasn't gotten back up to speed yet)
    pub up_to_speed: u8,

    /// calculated based on where the leader is, and adjusted by the desired column spacing and the column/position assignments
    pub goal_relative_distance: f64,

    /// a message for this participant to explain what is going on (untranslated; it will get run through translator on client machines)
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct TrackRules {
    // input only
    /// current time
    pub current_et: f64,
    /// current stage
    pub stage: i32,
    /// column assignment where pole position seems to be located
    pub pole_column: i32,

    /// whether yellow flag was requested or sum of participant mYellowSeverity's exceeds mSafetyCarThreshold
    pub yellow_flag_detected: u8,
    /// whether mYellowFlagLaps (below) is an admin request (0=no 1=yes 2=clear yellow)
    pub yellow_flag_laps_was_overridden: u8,

    /// whether safety car even exists
    pub safety_car_exists: u8,
    /// whether safety car is active
    pub safety_car_active: u8,
    /// number of laps
    pub safety_car_laps: i32,
    /// the threshold at which a safety car is called out (compared to the sum of TrackRulesParticipantV01::mYellowSeverity for each vehicle)
    pub safety_car_threshold: f32,
    /// safety car lap distance
    pub safety_car_lap_dist: f64,
    /// where the safety car starts from
    pub safety_car_lap_dist_at_start: f32,

    /// where the waypoint branch to the pits breaks off (this may not be perfectly accurate)
    pub pit_lane_start_dist: f32,
    /// the front of the teleport locations (a useful first guess as to where to throw the green flag)
    pub teleport_lap_dist: f32,

    // input/output
    /// see ScoringInfoV01 for values
    pub yellow_flag_state: i8,
    /// suggested number of laps to run under yellow (may be passed in with admin command)
    pub yellow_flag_laps: i16,

    pub safety_car_instruction: i32,
    /// maximum speed at which to drive
    pub safety_car_speed: f32,
    /// minimum spacing behind safety car (-1 to indicate no limit)
    pub safety_car_minimum_spacing: f32,
    /// maximum spacing behind safety car (-1 to indicate no limit)
    pub safety_car_maximum_spacing: f32,

    /// minimum desired spacing between vehicles in a column (-1 to indicate indeterminate/unenforced)
    pub minimum_column_spacing: f32,
    /// maximum desired spacing between vehicles in a column (-1 to indicate indeterminate/unenforced)
    pub maximum_column_spacing: f32,

    /// minimum speed that anybody should be driving (-1 to indicate no limit)
    pub minimum_speed: f32,
    /// maximum speed that anybody should be driving (-1 to indicate no limit)
    pub maximum_speed: f32,

    /// a message for everybody to explain what is going on (which will get run through translator on client machines)
    pub message: String,
}

/// Not supported yet.
#[derive(Clone, Debug)]
pub struct MultiRules {
    pub packet_id: PacketId,
}

/// Not supported yet.
#[derive(Clone, Debug)]
pub struct PitInfo {
    pub packet_id: PacketId,
}

/// Not supported yet.
#[derive(Clone, Debug)]
pub struct Weather {
    pub packet_id: PacketId,
}

#[derive(Clone, Debug)]
pub struct Extended {
    pub packet_id: PacketId,

    /// API version
    pub version: String,
    /// Is 64bit plugin?
    pub is64bit: u8,

    /// Physics options (updated on session start)
    pub physics: PhysicsOptions,

    /// Damage tracking for each vehicle
    ///
    /// Indexed by mID % MappedBufferHeader::MAX_MAPPED_IDS.
    pub tracked_damages: [TrackedDamage; MAX_MAPPED_IDS],

    // Function call based flags:
    /// in realtime as opposed to at the monitor (reported via last EnterRealtime/ExitRealtime calls).
    pub in_realtime_fc: u8,
    /// multimedia thread started (reported via ThreadStarted/ThreadStopped calls).
    pub multimedia_thread_started: u8,
    /// simulation thread started (reported via ThreadStarted/ThreadStopped calls).
    pub simulation_thread_started: u8,

    /// Set to true on Session Started, set to false on Session Ended.
    pub session_started: u8,
    /// Ticks when session started.
    pub ticks_session_started: i64,
    /// Ticks when session ended.
    pub ticks_session_ended: i64,
    /// Contains partial internals capture at session transition time.
    pub session_transition_capture: SessionTransitionCapture,

    /// Captured non-empty MessageInfoV01::mText message.
    pub displayed_message_update_capture: String,

    /// Direct Memory access stuff
    pub direct_memory_access_enabled: u8,

    /// Ticks when status message was updated,
    pub ticks_status_message_updated: i64,
    pub status_message: String,

    /// Ticks when last message history message was updated,
    pub ticks_last_history_message_updated: i64,
    pub last_history_message: String,

    /// speed limit m/s.
    pub current_pit_speed_limit: f32,

    /// Is Stock Car Rules plugin enabled?
    pub scr_plugin_enabled: u8,
    /// Stock Car Rules plugin DoubleFileType value, only meaningful if mSCRPluginEnabled is true.
    pub scr_plugin_double_file_type: i32,

    /// Ticks when last LSI phase message was updated.
    pub ticks_lsi_phase_message_updated: i64,
    pub lsi_phase_message: String,

    /// Ticks when last LSI pit state message was updated.
    pub ticks_lsi_pit_state_message_updated: i64,
    pub lsi_pit_state_message: String,

    /// Ticks when last LSI order instruction message was updated.
    pub ticks_lsi_order_instruction_message_updated: i64,
    pub lsi_order_instruction_message: String,

    /// Ticks when last FCY rules message was updated.  Currently, only SCR plugin sets that.
    pub ticks_lsi_rules_instruction_message_updated: i64,
    pub lsi_rules_instruction_message: String,
}

#[derive(Clone, Debug)]
pub struct PhysicsOptions {
    /// 0 (off) - 3 (high)
    pub traction_control: u8,
    /// 0 (off) - 2 (high)
    pub anti_lock_brakes: u8,
    /// 0 (off) - 2 (high)
    pub stability_control: u8,
    /// 0 (off), 1 (upshifts), 2 (downshifts), 3 (all)
    pub auto_shift: u8,
    /// 0 (off), 1 (on)
    pub auto_clutch: u8,
    /// 0 (off), 1 (on)
    pub invulnerable: u8,
    /// 0 (off), 1 (on)
    pub opposite_lock: u8,
    /// 0 (off) - 3 (high)
    pub steering_help: u8,
    /// 0 (off) - 2 (high)
    pub braking_help: u8,
    /// 0 (off), 1 (on)
    pub spin_recovery: u8,
    /// 0 (off), 1 (on)
    pub auto_pit: u8,
    /// 0 (off), 1 (on)
    pub auto_lift: u8,
    /// 0 (off), 1 (on)
    pub auto_blip: u8,

    /// fuel multiplier (0x-7x)
    pub fuel_mult: u8,
    /// tire wear multiplier (0x-7x)
    pub tire_mult: u8,
    /// mechanical failure setting, 0 (off), 1 (normal), 2 (timescaled)
    pub mech_fail: u8,
    /// 0 (off), 1 (on)
    pub allow_pitcrew_push: u8,
    /// accidental repeat shift prevention (0-5; see PLR file)
    pub repeat_shifts: u8,
    /// for auto-shifters at start of race: 0 (off), 1 (on)
    pub hold_clutch: u8,
    /// 0 (off), 1 (on)
    pub auto_reverse: u8,
    /// Whether shifting up and down simultaneously equals neutral
    pub alternate_neutral: u8,

    /// Whether player vehicle is currently under AI control
    pub ai_control: u8,

    /// time before auto-shifting can resume after recent manual shift
    pub manual_shift_override_time: f32,
    /// time before manual shifting can resume after recent auto shift
    pub auto_shift_override_time: f32,
    /// 0.0 (off) - 1.0
    pub speed_sensitive_steering: f32,
    /// speed (m/s) under which lock gets expanded to full
    pub steer_ratio_speed: f32,
}

#[derive(Clone, Debug)]
pub struct TrackedDamage {
    /// Max impact magnitude
    ///
    /// Tracked on every telemetry update, and reset on visit to pits or Session restart.
    pub max_impact_magnitude: f64,
    /// Accumulated impact magnitude
    ///
    /// Tracked on every telemetry update, and reset on visit to pits or Session restart.
    pub accumulated_impact_magnitude: f64,
}

#[derive(Clone, Debug)]
pub struct SessionTransitionCapture {
    pub game_phase: u8,
    pub session: i32,

    pub scoring_vehicles: Vec<VehScoringCapture>,
}

#[derive(Clone, Debug)]
pub struct VehScoringCapture {
    /// slot ID (note that it can be re-used in multiplayer after someone leaves)
    pub id: i32,
    pub place: u8,
    pub is_player: u8,
    pub finish_status: i8,
}

impl TryFrom<PageHeader> for PacketId {
    type Error = Error;

    fn try_from(header: PageHeader) -> Result<Self> {
        if header.version_update_begin != header.version_update_end {
            bail!("Page was in the process of being written")
        }
        Ok(PacketId(header.version_update_end))
    }
}

impl TryFrom<Box<PageForceFeedback>> for ForceFeedback {
    type Error = Error;

    fn try_from(value: Box<PageForceFeedback>) -> Result<Self> {
        let _packet_id: PacketId = value.ignored_header.try_into()?;
        Ok(Self {
            force_value: value.force_value,
        })
    }
}

impl TryFrom<Box<PageTelemetry>> for Telemetry {
    type Error = Error;

    fn try_from(value: Box<PageTelemetry>) -> Result<Telemetry> {
        let packet_id = value.header.try_into()?;
        let vehicle_count = value.num_vehicles.clamp(0, MAX_MAPPED_VEHICLES as i32) as usize;
        Ok(Self {
            packet_id,
            vehicles: value
                .vehicles
                .iter()
                .take(vehicle_count)
                .map(|v| v.into())
                .collect(),
        })
    }
}

impl From<&PageVehicleTelemetry> for VehicleTelemetry {
    fn from(value: &PageVehicleTelemetry) -> Self {
        Self {
            id: value.id,
            delta_time: value.delta_time,
            elapsed_time: value.elapsed_time,
            lap_number: value.lap_number,
            lap_start_et: value.lap_start_et,
            vehicle_name: cp1252_to_string(&value.vehicle_name).unwrap_or_default(),
            track_name: cp1252_to_string(&value.track_name).unwrap_or_default(),
            pos: value.pos.into(),
            local_vel: value.local_vel.into(),
            local_accel: value.local_accel.into(),
            ori: value.ori.map(Into::into),
            local_rot: value.local_rot.into(),
            local_rot_accel: value.local_rot_accel.into(),
            gear: value.gear,
            engine_rpm: value.engine_rpm,
            engine_water_temp: value.engine_water_temp,
            engine_oil_temp: value.engine_oil_temp,
            clutch_rpm: value.clutch_rpm,
            unfiltered_throttle: value.unfiltered_throttle,
            unfiltered_brake: value.unfiltered_brake,
            unfiltered_steering: value.unfiltered_steering,
            unfiltered_clutch: value.unfiltered_clutch,
            filtered_throttle: value.filtered_throttle,
            filtered_brake: value.filtered_brake,
            filtered_steering: value.filtered_steering,
            filtered_clutch: value.filtered_clutch,
            steering_shaft_torque: value.steering_shaft_torque,
            front3rd_deflection: value.front3rd_deflection,
            rear3rd_deflection: value.rear3rd_deflection,
            front_wing_height: value.front_wing_height,
            front_ride_height: value.front_ride_height,
            rear_ride_height: value.rear_ride_height,
            drag: value.drag,
            front_downforce: value.front_downforce,
            rear_downforce: value.rear_downforce,
            fuel: value.fuel,
            engine_max_rpm: value.engine_max_rpm,
            scheduled_stops: value.scheduled_stops,
            overheating: value.overheating,
            detached: value.detached,
            headlights: value.headlights,
            dent_severity: value.dent_severity,
            last_impact_et: value.last_impact_et,
            last_impact_magnitude: value.last_impact_magnitude,
            last_impact_pos: value.last_impact_pos.into(),
            engine_torque: value.engine_torque,
            current_sector: value.current_sector,
            speed_limiter: value.speed_limiter,
            max_gears: value.max_gears,
            front_tire_compound_index: value.front_tire_compound_index,
            rear_tire_compound_index: value.rear_tire_compound_index,
            fuel_capacity: value.fuel_capacity,
            front_flap_activated: value.front_flap_activated,
            rear_flap_activated: value.rear_flap_activated,
            rear_flap_legal_status: value.rear_flap_legal_status,
            ignition_starter: value.ignition_starter,
            front_tire_compound_name: cp1252_to_string(&value.front_tire_compound_name)
                .unwrap_or_default(),
            rear_tire_compound_name: cp1252_to_string(&value.rear_tire_compound_name)
                .unwrap_or_default(),
            speed_limiter_available: value.speed_limiter_available,
            anti_stall_activated: value.anti_stall_activated,
            visual_steering_wheel_range: value.visual_steering_wheel_range,
            rear_brake_bias: value.rear_brake_bias,
            turbo_boost_pressure: value.turbo_boost_pressure,
            physics_to_graphics_offset: value.physics_to_graphics_offset,
            physical_steering_wheel_range: value.physical_steering_wheel_range,
            wheels: value.wheels.map(|v| (&v).into()),
        }
    }
}

impl From<&PageWheelTelemetry> for WheelTelemetry {
    fn from(value: &PageWheelTelemetry) -> Self {
        Self {
            suspension_deflection: value.suspension_deflection,
            ride_height: value.ride_height,
            susp_force: value.susp_force,
            brake_temp: value.brake_temp,
            brake_pressure: value.brake_pressure,
            rotation: value.rotation,
            lateral_patch_vel: value.lateral_patch_vel,
            longitudinal_patch_vel: value.longitudinal_patch_vel,
            lateral_ground_vel: value.lateral_ground_vel,
            longitudinal_ground_vel: value.longitudinal_ground_vel,
            camber: value.camber,
            lateral_force: value.lateral_force,
            longitudinal_force: value.longitudinal_force,
            tire_load: value.tire_load,
            grip_fract: value.grip_fract,
            pressure: value.pressure,
            temperature: value.temperature,
            wear: value.wear,
            terrain_name: cp1252_to_string(&value.terrain_name).unwrap_or_default(),
            surface_type: value.surface_type,
            flat: value.flat,
            detached: value.detached,
            static_undeflected_radius: value.static_undeflected_radius,
            vertical_tire_deflection: value.vertical_tire_deflection,
            wheel_ylocation: value.wheel_ylocation,
            toe: value.toe,
            tire_carcass_temperature: value.tire_carcass_temperature,
            tire_inner_layer_temperature: value.tire_inner_layer_temperature,
        }
    }
}

impl TryFrom<Box<PageScoring>> for Scoring {
    type Error = Error;

    fn try_from(value: Box<PageScoring>) -> Result<Scoring> {
        let packet_id = value.header.try_into()?;
        let scoring_info: ScoringInfo = (&value.scoring_info).into();
        let vehicles = value
            .vehicles
            .iter()
            .take(
                value
                    .scoring_info
                    .num_vehicles
                    .clamp(0, MAX_MAPPED_VEHICLES as i32) as usize,
            )
            .map(Into::into)
            .collect();
        Ok(Self {
            packet_id,
            scoring_info,
            vehicles,
        })
    }
}

impl From<&PageScoringInfo> for ScoringInfo {
    fn from(value: &PageScoringInfo) -> Self {
        Self {
            track_name: cp1252_to_string(&value.track_name).unwrap_or_default(),
            session: value.session,
            current_et: value.current_et,
            end_et: value.end_et,
            max_laps: value.max_laps,
            lap_dist: value.lap_dist,
            game_phase: value.game_phase,
            yellow_flag_state: value.yellow_flag_state,
            sector_flag: value.sector_flag,
            start_light: value.start_light,
            num_red_lights: value.num_red_lights,
            in_realtime: value.in_realtime,
            player_name: cp1252_to_string(&value.player_name).unwrap_or_default(),
            plr_file_name: cp1252_to_string(&value.plr_file_name).unwrap_or_default(),
            dark_cloud: value.dark_cloud,
            raining: value.raining,
            ambient_temp: value.ambient_temp,
            track_temp: value.track_temp,
            wind: value.wind.into(),
            min_path_wetness: value.min_path_wetness,
            max_path_wetness: value.max_path_wetness,
            game_mode: value.game_mode,
            is_password_protected: value.is_password_protected,
            server_port: value.server_port,
            server_public_ip: value.server_public_ip,
            max_players: value.max_players,
            server_name: cp1252_to_string(&value.server_name).unwrap_or_default(),
            start_et: value.start_et,
            avg_path_wetness: value.avg_path_wetness,
        }
    }
}

impl From<&PageVehicleScoring> for VehicleScoring {
    fn from(value: &PageVehicleScoring) -> Self {
        Self {
            id: value.id,
            driver_name: cp1252_to_string(&value.driver_name).unwrap_or_default(),
            vehicle_name: cp1252_to_string(&value.vehicle_name).unwrap_or_default(),
            total_laps: value.total_laps,
            sector: value.sector,
            finish_status: value.finish_status,
            lap_dist: value.lap_dist,
            path_lateral: value.path_lateral,
            track_edge: value.track_edge,
            best_sector1: value.best_sector1,
            best_sector2: value.best_sector2,
            best_lap_time: value.best_lap_time,
            last_sector1: value.last_sector1,
            last_sector2: value.last_sector2,
            last_lap_time: value.last_lap_time,
            cur_sector1: value.cur_sector1,
            cur_sector2: value.cur_sector2,
            num_pitstops: value.num_pitstops,
            num_penalties: value.num_penalties,
            is_player: value.is_player,
            control: value.control,
            in_pits: value.in_pits,
            place: value.place,
            vehicle_class: cp1252_to_string(&value.vehicle_class).unwrap_or_default(),
            time_behind_next: value.time_behind_next,
            laps_behind_next: value.laps_behind_next,
            time_behind_leader: value.time_behind_leader,
            laps_behind_leader: value.laps_behind_leader,
            lap_start_et: value.lap_start_et,
            pos: Default::default(),
            local_vel: Default::default(),
            local_accel: Default::default(),
            ori: value.ori.map(Into::into),
            local_rot: Default::default(),
            local_rot_accel: Default::default(),
            headlights: value.headlights,
            pit_state: value.pit_state,
            server_scored: value.server_scored,
            individual_phase: value.individual_phase,
            qualification: value.qualification,
            time_into_lap: value.time_into_lap,
            estimated_lap_time: value.estimated_lap_time,
            pit_group: cp1252_to_string(&value.pit_group).unwrap_or_default(),
            flag: value.flag,
            under_yellow: value.under_yellow,
            count_lap_flag: value.count_lap_flag,
            in_garage_stall: value.in_garage_stall,
            upgrade_pack: cp1252_to_string(&value.upgrade_pack).unwrap_or_default(),
            pit_lap_dist: value.pit_lap_dist,
            best_lap_sector1: value.best_lap_sector1,
            best_lap_sector2: value.best_lap_sector2,
        }
    }
}

impl TryFrom<Box<PageRules>> for Rules {
    type Error = Error;

    fn try_from(value: Box<PageRules>) -> Result<Rules> {
        let packet_id = value.header.try_into()?;
        let track_rules: TrackRules = (&value.track_rules).into();
        let actions = value
            .actions
            .iter()
            .take(
                value
                    .track_rules
                    .num_actions
                    .clamp(0, MAX_MAPPED_VEHICLES as i32) as usize,
            )
            .map(Into::into)
            .collect();
        let participants = value
            .participants
            .iter()
            .take(
                value
                    .track_rules
                    .num_participants
                    .clamp(0, MAX_MAPPED_VEHICLES as i32) as usize,
            )
            .map(Into::into)
            .collect();
        Ok(Self {
            packet_id,
            track_rules,
            actions,
            participants,
        })
    }
}

impl From<&PageTrackRules> for TrackRules {
    fn from(value: &PageTrackRules) -> Self {
        Self {
            current_et: value.current_et,
            stage: value.stage,
            pole_column: value.pole_column,
            yellow_flag_detected: value.yellow_flag_detected,
            yellow_flag_laps_was_overridden: value.yellow_flag_laps_was_overridden,
            safety_car_exists: value.safety_car_exists,
            safety_car_active: value.safety_car_active,
            safety_car_laps: value.safety_car_laps,
            safety_car_threshold: value.safety_car_threshold,
            safety_car_lap_dist: value.safety_car_lap_dist,
            safety_car_lap_dist_at_start: value.safety_car_lap_dist_at_start,
            pit_lane_start_dist: value.pit_lane_start_dist,
            teleport_lap_dist: value.teleport_lap_dist,
            yellow_flag_state: value.yellow_flag_state,
            yellow_flag_laps: value.yellow_flag_laps,
            safety_car_instruction: value.safety_car_instruction,
            safety_car_speed: value.safety_car_speed,
            safety_car_minimum_spacing: value.safety_car_minimum_spacing,
            safety_car_maximum_spacing: value.safety_car_maximum_spacing,
            minimum_column_spacing: value.minimum_column_spacing,
            maximum_column_spacing: value.maximum_column_spacing,
            minimum_speed: value.minimum_speed,
            maximum_speed: value.maximum_speed,
            message: cp1252_to_string(&value.message).unwrap_or_default(),
        }
    }
}

impl From<&PageTrackRulesAction> for TrackRulesAction {
    fn from(value: &PageTrackRulesAction) -> Self {
        Self {
            command: value.command,
            id: value.id,
            elapsed_time: value.elapsed_time,
        }
    }
}

impl From<&PageTrackRulesParticipant> for TrackRulesParticipant {
    fn from(value: &PageTrackRulesParticipant) -> Self {
        Self {
            id: value.id,
            frozen_order: value.frozen_order,
            place: value.place,
            yellow_severity: value.yellow_severity,
            current_relative_distance: value.current_relative_distance,
            relative_laps: value.relative_laps,
            column_assignment: value.column_assignment,
            position_assignment: value.position_assignment,
            pits_open: value.pits_open,
            up_to_speed: value.up_to_speed,
            goal_relative_distance: value.goal_relative_distance,
            message: cp1252_to_string(&value.message).unwrap_or_default(),
        }
    }
}

impl TryFrom<Box<PageMultiRules>> for MultiRules {
    type Error = Error;

    fn try_from(value: Box<PageMultiRules>) -> Result<MultiRules> {
        let packet_id = value.header.try_into()?;
        Ok(Self { packet_id })
    }
}

impl TryFrom<Box<PagePitInfo>> for PitInfo {
    type Error = Error;

    fn try_from(value: Box<PagePitInfo>) -> Result<PitInfo> {
        let packet_id = value.header.try_into()?;
        Ok(Self { packet_id })
    }
}

impl TryFrom<Box<PageWeather>> for Weather {
    type Error = Error;

    fn try_from(value: Box<PageWeather>) -> Result<Weather> {
        let packet_id = value.header.try_into()?;
        Ok(Self { packet_id })
    }
}

impl TryFrom<Box<PageExtended>> for Extended {
    type Error = Error;

    fn try_from(value: Box<PageExtended>) -> Result<Extended> {
        let packet_id = value.header.try_into()?;
        Ok(Self {
            packet_id,
            version: cp1252_to_string(&value.version).unwrap_or_default(),
            is64bit: value.is64bit,
            physics: (&value.physics).into(),
            tracked_damages: value.tracked_damages.map(|v| (&v).into()),
            in_realtime_fc: value.in_realtime_fc,
            multimedia_thread_started: value.multimedia_thread_started,
            simulation_thread_started: value.simulation_thread_started,
            session_started: value.session_started,
            ticks_session_started: value.ticks_session_started,
            ticks_session_ended: value.ticks_session_ended,
            session_transition_capture: (&value.session_transition_capture).into(),
            displayed_message_update_capture: cp1252_to_string(
                &value.displayed_message_update_capture,
            )
            .unwrap_or_default(),
            direct_memory_access_enabled: value.direct_memory_access_enabled,
            ticks_status_message_updated: value.ticks_status_message_updated,
            status_message: cp1252_to_string(&value.status_message).unwrap_or_default(),
            ticks_last_history_message_updated: value.ticks_last_history_message_updated,
            last_history_message: cp1252_to_string(&value.last_history_message).unwrap_or_default(),
            current_pit_speed_limit: value.current_pit_speed_limit,
            scr_plugin_enabled: value.scr_plugin_enabled,
            scr_plugin_double_file_type: value.scr_plugin_double_file_type,
            ticks_lsi_phase_message_updated: value.ticks_lsi_phase_message_updated,
            lsi_phase_message: cp1252_to_string(&value.lsi_phase_message).unwrap_or_default(),
            ticks_lsi_pit_state_message_updated: value.ticks_lsi_pit_state_message_updated,
            lsi_pit_state_message: cp1252_to_string(&value.lsi_pit_state_message)
                .unwrap_or_default(),
            ticks_lsi_order_instruction_message_updated: value
                .ticks_lsi_order_instruction_message_updated,
            lsi_order_instruction_message: cp1252_to_string(&value.lsi_order_instruction_message)
                .unwrap_or_default(),
            ticks_lsi_rules_instruction_message_updated: value
                .ticks_lsi_rules_instruction_message_updated,
            lsi_rules_instruction_message: cp1252_to_string(&value.lsi_rules_instruction_message)
                .unwrap_or_default(),
        })
    }
}

impl From<&PagePhysicsOptions> for PhysicsOptions {
    fn from(value: &PagePhysicsOptions) -> Self {
        Self {
            traction_control: value.traction_control,
            anti_lock_brakes: value.anti_lock_brakes,
            stability_control: value.stability_control,
            auto_shift: value.auto_shift,
            auto_clutch: value.auto_clutch,
            invulnerable: value.invulnerable,
            opposite_lock: value.opposite_lock,
            steering_help: value.steering_help,
            braking_help: value.braking_help,
            spin_recovery: value.spin_recovery,
            auto_pit: value.auto_pit,
            auto_lift: value.auto_lift,
            auto_blip: value.auto_blip,
            fuel_mult: value.fuel_mult,
            tire_mult: value.tire_mult,
            mech_fail: value.mech_fail,
            allow_pitcrew_push: value.allow_pitcrew_push,
            repeat_shifts: value.repeat_shifts,
            hold_clutch: value.hold_clutch,
            auto_reverse: value.auto_reverse,
            alternate_neutral: value.alternate_neutral,
            ai_control: value.ai_control,
            manual_shift_override_time: value.manual_shift_override_time,
            auto_shift_override_time: value.auto_shift_override_time,
            speed_sensitive_steering: value.speed_sensitive_steering,
            steer_ratio_speed: value.steer_ratio_speed,
        }
    }
}

impl From<&PageTrackedDamage> for TrackedDamage {
    fn from(value: &PageTrackedDamage) -> Self {
        Self {
            max_impact_magnitude: value.max_impact_magnitude,
            accumulated_impact_magnitude: value.accumulated_impact_magnitude,
        }
    }
}

impl From<&PageSessionTransitionCapture> for SessionTransitionCapture {
    fn from(value: &PageSessionTransitionCapture) -> Self {
        let scoring_vehicles = value
            .scoring_vehicles
            .iter()
            .take(
                value
                    .num_scoring_vehicles
                    .clamp(0, MAX_MAPPED_VEHICLES as i32) as usize,
            )
            .map(Into::into)
            .collect();
        Self {
            game_phase: value.game_phase,
            session: value.session,
            scoring_vehicles,
        }
    }
}

impl From<&PageVehScoringCapture> for VehScoringCapture {
    fn from(value: &PageVehScoringCapture) -> Self {
        Self {
            id: value.id,
            place: value.place,
            is_player: value.is_player,
            finish_status: value.finish_status,
        }
    }
}

impl From<PageVec3> for Vec3 {
    fn from(value: PageVec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
