pub const MAX_MAPPED_VEHICLES: usize = 128;
pub const MAX_MAPPED_IDS: usize = 512;

type String128 = [u8; 128];
type String96 = [u8; 96];
type String64 = [u8; 64];
type String32 = [u8; 32];
type String24 = [u8; 24];
type String18 = [u8; 18];
type String16 = [u8; 16];
type String12 = [u8; 12];
type Garbage = u8;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default)]
pub struct PageVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageHeader {
    /// Incremented right before buffer is written to.
    pub version_update_begin: u32,
    /// Incremented after buffer write is done.
    pub version_update_end: u32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageForceFeedback {
    pub ignored_header: PageHeader,

    /// Current FFB value reported via InternalsPlugin::ForceFeedback.
    pub force_value: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageTelemetry {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,

    /// current number of vehicles
    pub num_vehicles: i32,
    pub vehicles: [PageVehicleTelemetry; MAX_MAPPED_VEHICLES],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageVehicleTelemetry {
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
    pub vehicle_name: String64,
    /// current track name    
    pub track_name: String64,

    // Position and derivatives
    /// world position in meters    
    pub pos: PageVec3,
    /// velocity (meters/sec) in local vehicle coordinates    
    pub local_vel: PageVec3,
    /// acceleration (meters/sec^2) in local vehicle coordinates    
    pub local_accel: PageVec3,

    // Orientation and derivatives
    /// rows of orientation matrix (use TelemQuat conversions if desired), also converts local    
    pub ori: [PageVec3; 3],
    // vehicle vectors into world X, Y, or Z using dot product of rows 0, 1, or 2 respectively
    /// rotation (radians/sec) in local vehicle coordinates    
    pub local_rot: PageVec3,
    /// rotational acceleration (radians/sec^2) in local vehicle coordinates    
    pub local_rot_accel: PageVec3,

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
    pub last_impact_pos: PageVec3,

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
    pub front_tire_compound_name: String18,
    /// name of rear tire compound    
    pub rear_tire_compound_name: String18,

    /// whether speed limiter is available    
    pub speed_limiter_available: u8,
    /// whether (hard) anti-stall is activated    
    pub anti_stall_activated: u8,
    ///
    unused: [Garbage; 2],
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

    // Future use
    /// for future use (note that the slot ID has been moved to mID above)    
    expansion: [Garbage; 152],

    // keeping this at the end of the structure to make it easier to replace in future versions
    /// wheel info (front left, front right, rear left, rear right)    
    pub wheels: [PageWheelTelemetry; 4],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageWheelTelemetry {
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
    pub terrain_name: String16,
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

    /// for future use
    expansion: [Garbage; 24],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageScoring {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,

    pub scoring_info: PageScoringInfo,

    pub vehicles: [PageVehicleScoring; MAX_MAPPED_VEHICLES],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageScoringInfo {
    /// current track name
    pub track_name: String64,
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
    /// results stream additions since last update (newline-delimited and NULL-terminated)
    pointer1: [Garbage; 8],

    /// current number of vehicles
    pub num_vehicles: i32,

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
    pub player_name: String32,
    /// may be encoded to be a legal filename
    pub plr_file_name: String64,

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
    pub wind: PageVec3,
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
    pub server_name: String32,
    /// start time (seconds since midnight) of the event
    pub start_et: f32,

    /// average wetness on main path 0.0-1.0
    pub avg_path_wetness: f64,

    /// Future use
    expansion: [Garbage; 200],

    /// array of vehicle scoring info's
    pointer2: [Garbage; 8],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageVehicleScoring {
    /// slot ID (note that it can be re-used in multiplayer after someone leaves)
    pub id: i32,
    /// driver name
    pub driver_name: String32,
    /// vehicle name
    pub vehicle_name: String64,
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
    pub vehicle_class: String32,

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
    pub pos: PageVec3,
    /// velocity (meters/sec) in local vehicle coordinates    
    pub local_vel: PageVec3,
    /// acceleration (meters/sec^2) in local vehicle coordinates    
    pub local_accel: PageVec3,

    // Orientation and derivatives
    /// rows of orientation matrix (use TelemQuat conversions if desired), also converts local    
    pub ori: [PageVec3; 3],
    // vehicle vectors into world X, Y, or Z using dot product of rows 0, 1, or 2 respectively
    /// rotation (radians/sec) in local vehicle coordinates    
    pub local_rot: PageVec3,
    /// rotational acceleration (radians/sec^2) in local vehicle coordinates    
    pub local_rot_accel: PageVec3,

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
    pub pit_group: String24,
    /// primary flag being shown to vehicle (currently only 0 = green or 6 = blue)
    pub flag: u8,
    /// whether this car has taken a full-course caution flag at the start/finish line    
    pub under_yellow: u8,
    pub count_lap_flag: u8,
    /// appears to be within the correct garage stall    
    pub in_garage_stall: u8,

    /// Coded upgrades    
    pub upgrade_pack: String16,

    /// location of pit in terms of lap distance    
    pub pit_lap_dist: f32,

    /// sector 1 time from best lap (not necessarily the best sector 1 time)    
    pub best_lap_sector1: f32,
    /// sector 2 time from best lap (not necessarily the best sector 2 time)    
    pub best_lap_sector2: f32,

    /// for future use    
    expansion: [Garbage; 48],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageRules {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,
    pub track_rules: PageTrackRules,
    pub actions: [PageTrackRulesAction; MAX_MAPPED_VEHICLES],
    pub participants: [PageTrackRulesParticipant; MAX_MAPPED_VEHICLES],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageTrackRulesAction {
    // input only
    /// recommended action
    pub command: i32,
    /// slot ID if applicable
    pub id: i32,
    /// elapsed time that event occurred, if applicable
    pub elapsed_time: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageTrackRulesParticipant {
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

    unused: [Garbage; 2],

    /// calculated based on where the leader is, and adjusted by the desired column spacing and the column/position assignments
    pub goal_relative_distance: f64,

    /// a message for this participant to explain what is going on (untranslated; it will get run through translator on client machines)
    pub message: String96,

    /// future expansion
    expansion: [Garbage; 192],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageTrackRules {
    // input only
    /// current time
    pub current_et: f64,
    /// current stage
    pub stage: i32,
    /// column assignment where pole position seems to be located
    pub pole_column: i32,
    /// number of recent actions
    pub num_actions: i32,

    /// array of recent actions
    pointer1: [Garbage; 8],

    /// number of participants (vehicles)
    pub num_participants: i32,

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

    /// future input expansion
    input_expansion: [Garbage; 256],

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
    pub message: String96,

    /// array of partipants (vehicles)
    pointer2: [Garbage; 8],

    /// future input/output expansion
    input_output_expansion: [Garbage; 256],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageMultiRules {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PagePitInfo {
    pub header: PageHeader,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageWeather {
    pub header: PageHeader,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageExtended {
    pub header: PageHeader,

    /// API version
    pub version: String12,
    /// Is 64bit plugin?
    pub is64bit: u8,

    /// Physics options (updated on session start)
    pub physics: PagePhysicsOptions,

    /// Damage tracking for each vehicle
    ///
    /// Indexed by mID % MappedBufferHeader::MAX_MAPPED_IDS.
    pub tracked_damages: [PageTrackedDamage; MAX_MAPPED_IDS],

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
    pub session_transition_capture: PageSessionTransitionCapture,

    /// Captured non-empty MessageInfoV01::mText message.
    pub displayed_message_update_capture: String128,

    /// Direct Memory access stuff
    pub direct_memory_access_enabled: u8,

    /// Ticks when status message was updated,
    pub ticks_status_message_updated: i64,
    pub status_message: String128,

    /// Ticks when last message history message was updated,
    pub ticks_last_history_message_updated: i64,
    pub last_history_message: String128,

    /// speed limit m/s.
    pub current_pit_speed_limit: f32,

    /// Is Stock Car Rules plugin enabled?
    pub scr_plugin_enabled: u8,
    /// Stock Car Rules plugin DoubleFileType value, only meaningful if mSCRPluginEnabled is true.
    pub scr_plugin_double_file_type: i32,

    /// Ticks when last LSI phase message was updated.
    pub ticks_lsi_phase_message_updated: i64,
    pub lsi_phase_message: String96,

    /// Ticks when last LSI pit state message was updated.
    pub ticks_lsi_pit_state_message_updated: i64,
    pub lsi_pit_state_message: String96,

    /// Ticks when last LSI order instruction message was updated.
    pub ticks_lsi_order_instruction_message_updated: i64,
    pub lsi_order_instruction_message: String96,

    /// Ticks when last FCY rules message was updated.  Currently, only SCR plugin sets that.
    pub ticks_lsi_rules_instruction_message_updated: i64,
    pub lsi_rules_instruction_message: String96,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PagePhysicsOptions {
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
    unused1: Garbage,
    unused2: Garbage,

    /// time before auto-shifting can resume after recent manual shift
    pub manual_shift_override_time: f32,
    /// time before manual shifting can resume after recent auto shift
    pub auto_shift_override_time: f32,
    /// 0.0 (off) - 1.0
    pub speed_sensitive_steering: f32,
    /// speed (m/s) under which lock gets expanded to full
    pub steer_ratio_speed: f32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageTrackedDamage {
    /// Max impact magnitude
    ///
    /// Tracked on every telemetry update, and reset on visit to pits or Session restart.
    pub max_impact_magnitude: f64,
    /// Accumulated impact magnitude
    ///
    /// Tracked on every telemetry update, and reset on visit to pits or Session restart.
    pub accumulated_impact_magnitude: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageSessionTransitionCapture {
    pub game_phase: u8,
    pub session: i32,

    pub num_scoring_vehicles: i32,
    pub scoring_vehicles: [PageVehScoringCapture; MAX_MAPPED_VEHICLES],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug)]
pub struct PageVehScoringCapture {
    /// slot ID (note that it can be re-used in multiplayer after someone leaves)
    pub id: i32,
    pub place: u8,
    pub is_player: u8,
    pub finish_status: i8,
}
