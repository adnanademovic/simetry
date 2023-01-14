use typename::TypeName;

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct PenaltyRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct StatusRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct SessionTypeRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct FlagTypeRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct TrackGripStatusRaw {
    pub data: i32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug)]
pub struct RainIntensityRaw {
    pub data: i32,
}

/// Data changes at each graphic step.
///
/// All data refers to the player’s car.
#[repr(C, packed(4))]
#[derive(Clone, Debug, TypeName)]
pub struct PageFilePhysics {
    /// Current step index
    pub packet_id: i32,
    /// Gas pedal input value (from -0 to 1.0)
    pub gas: f32,
    /// Brake pedal input value (from -0 to 1.0)
    pub brake: f32,
    /// Amount of fuel remaining in kg
    pub fuel: f32,
    /// Current gear
    pub gear: i32,
    /// Engine revolutions per minute
    pub rpm: i32,
    /// Steering input value (from -1.0 to 1.0)
    pub steer_angle: f32,
    /// Car speed in km/h
    pub speed_kmh: f32,
    /// Car velocity vector in global coordinates
    pub velocity: [f32; 3],
    /// Car acceleration vector in global coordinates
    pub acc_g: [f32; 3],
    /// Tyre slip for each tyre [FL, FR, RL, RR]
    pub wheel_slip: [f32; 4],
    /// Wheel load for each tyre [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub wheel_load: [f32; 4],
    /// Tyre pressure [FL, FR, RL, RR]
    pub wheels_pressure: [f32; 4],
    /// Wheel angular speed in rad/s [FL, FR, RL, RR]
    pub wheel_angular_speed: [f32; 4],
    /// Tyre wear [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_wear: [f32; 4],
    /// Dirt accumulated on tyre surface [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_dirty_level: [f32; 4],
    /// Tyre rubber core temperature [FL, FR, RL, RR]
    pub tyre_core_temperature: [f32; 4],
    /// Wheels camber in radians [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub camber_rad: [f32; 4],
    /// Suspension travel [FL, FR, RL, RR]
    pub suspension_travel: [f32; 4],
    /// DRS on *(NOT SENT BY SIM)*
    pub drs: f32,
    /// TC in action
    pub tc: f32,
    /// Car yaw orientation
    pub heading: f32,
    /// Car pitch orientation
    pub pitch: f32,
    /// Car roll orientation
    pub roll: f32,
    /// Centre of gravity height *(NOT SENT BY SIM)*
    pub cg_height: f32,
    /// Car damage: front 0, rear 1, left 2, right 3, centre 4
    pub car_damage: [f32; 5],
    /// Number of tyres out of track *(NOT SENT BY SIM)*
    pub number_of_tyres_out: i32,
    /// Pit limiter is on
    pub pit_limiter_on: i32,
    /// ABS in action
    pub abs: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_charge: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_input: f32,
    /// Automatic transmission on
    pub auto_shifter_on: i32,
    /// Ride height: 0 front, 1 rear *(NOT SENT BY SIM)*
    pub ride_height: [f32; 2],
    /// Car turbo level
    pub turbo_boost: f32,
    /// Car ballast in kg / Not implemented *(NOT SENT BY SIM)*
    pub ballast: f32,
    /// Air density *(NOT SENT BY SIM)*
    pub air_density: f32,
    /// Air temperature
    pub air_temp: f32,
    /// Road temperature
    pub road_temp: f32,
    /// Car angular velocity vector in local coordinates
    pub local_angular_vel: [f32; 3],
    /// Force feedback signal
    pub final_ff: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub performance_meter: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub engine_brake: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_recovery_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_power_level: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_heat_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub ers_is_charging: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub kers_current_kj: f32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_available: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub drs_enabled: i32,
    /// Brake discs temperatures
    pub brake_temp: [f32; 4],
    /// Clutch pedal input value (from -0 to 1.0)
    pub clutch: f32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_i: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_m: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub tyre_temp_o: [f32; 4],
    /// Car is controlled by the AI
    pub is_ai_controlled: i32,
    /// Tyre contact point global coordinates [FL, FR, RL, RR]
    pub tyre_contact_point: [[f32; 3]; 4],
    /// Tyre contact normal [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_normal: [[f32; 3]; 4],
    /// Tyre contact heading [FL, FR, RL, RR] [x,y,z]
    pub tyre_contact_heading: [[f32; 3]; 4],
    /// Front brake bias, see Appendix 4
    pub brake_bias: f32,
    /// Car velocity vector in local coordinates
    pub local_velocity: [f32; 3],
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_activations: i32,
    /// Not used in ACC *(NOT SENT BY SIM)*
    pub p2p_status: i32,
    /// Maximum engine rpm *(NOT SENT BY SIM)*
    pub current_max_rpm: i32,
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub mz: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fx: [f32; 4],
    /// Not shown in ACC *(NOT SENT BY SIM)*
    pub fy: [f32; 4],
    /// Tyre slip ratio [FL, FR, RL, RR] in radians
    pub slip_ratio: [f32; 4],
    /// Tyre slip angle [FL, FR, RL, RR]
    pub slip_angle: [f32; 4],
    /// TC in action *(NOT SENT BY SIM)*
    pub tc_in_action: i32,
    /// ABS in action *(NOT SENT BY SIM)*
    pub abs_in_action: i32,
    /// Suspensions damage levels [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub suspension_damage: [f32; 4],
    /// Tyres core temperatures [FL, FR, RL, RR] *(NOT SENT BY SIM)*
    pub tyre_temp: [f32; 4],
    /// Water Temperature
    pub water_temp: f32,
    /// Brake pressure [FL, FR, RL, RR] see Appendix 2
    pub brake_pressure: [f32; 4],
    /// Brake pad compund front
    pub front_brake_compound: i32,
    /// Brake pad compund rear
    pub rear_brake_compound: i32,
    /// Brake pad wear [FL, FR, RL, RR]
    pub pad_life: [f32; 4],
    /// Brake disk wear [FL, FR, RL, RR]
    pub disc_life: [f32; 4],
    /// Ignition switch set to on?
    pub ignition_on: i32,
    /// Starter Switch set to on?
    pub starter_engine_on: i32,
    /// Engine running?
    pub is_engine_running: i32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub kerb_vibration: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub slip_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub g_vibrations: f32,
    /// Vibrations sent to the FFB, could be used for motion rigs
    pub abs_vibrations: f32,
}

/// Data updated at each graphical step.
///
/// They mostly refer to player’s car except for carCoordinates and carID,
/// which refer to the cars currently on track.
#[repr(C, packed(4))]
#[derive(Clone, Debug, TypeName)]
pub struct PageFileGraphics {
    /// Current step index
    pub packet_id: i32,
    /// See enums ACC_STATUS
    pub status: StatusRaw,
    /// See enums ACC_SESSION_TYPE
    pub session: SessionTypeRaw,
    /// Current lap time in wide character
    pub current_time: [u16; 15],
    /// Last lap time in wide character
    pub last_time: [u16; 15],
    /// Best lap time in wide character
    pub best_time: [u16; 15],
    /// Last split time in wide character
    pub split: [u16; 15],
    /// No of completed laps
    pub completed_laps: i32,
    /// Current player position
    pub position: i32,
    /// Current lap time in milliseconds
    pub i_current_time: i32,
    /// Last lap time in milliseconds
    pub i_last_time: i32,
    /// Best lap time in milliseconds
    pub i_best_time: i32,
    /// Session time left
    pub session_time_left: f32,
    /// Distance travelled in the current stint
    pub distance_traveled: f32,
    /// Car is pitting
    pub is_in_pit: i32,
    /// Current track sector
    pub current_sector_index: i32,
    /// Last sector time in milliseconds
    pub last_sector_time: i32,
    /// Number of completed laps
    pub number_of_laps: i32,
    /// Tyre compound used
    pub tyre_compound: [u16; 33],
    /// Not used in ACC
    pub replay_time_multiplier: f32,
    /// Car position on track spline (0.0 start to 1.0 finish)
    pub normalized_car_position: f32,
    /// Number of cars on track
    pub active_cars: i32,
    /// Coordinates of cars on track
    pub car_coordinates: [[f32; 3]; 60],
    /// Car IDs of cars on track
    pub car_id: [i32; 60],
    /// Player Car ID
    pub player_car_id: i32,
    /// Penalty time to wait
    pub penalty_time: f32,
    /// See enums ACC_FLAG_TYPE
    pub flag: FlagTypeRaw,
    /// See enums ACC_PENALTY_TYPE
    pub penalty: PenaltyRaw,
    /// Ideal line on
    pub ideal_line_on: i32,
    /// Car is in pit lane
    pub is_in_pit_lane: i32,
    /// Ideal line friction coefficient
    pub surface_grip: f32,
    /// Mandatory pit is completed
    pub mandatory_pit_done: i32,
    /// Wind speed in m/s
    pub wind_speed: f32,
    /// wind direction in radians
    pub wind_direction: f32,
    /// Car is working on setup
    pub is_setup_menu_visible: i32,
    /// current car main display index, see Appendix 1
    pub main_display_index: i32,
    /// current car secondary display index
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
    pub rain_lights: i32,
    /// Flashing lights on
    pub flashing_lights: i32,
    /// Current lights stage
    pub lights_stage: i32,
    /// Exhaust temperature
    pub exhaust_temperature: f32,
    /// Current wiper stage
    pub wiper_lv: i32,
    /// Time the driver is allowed to drive/race (ms)
    pub driver_stint_total_time_left: i32,
    /// Time the driver is allowed to drive/stint (ms)
    pub driver_stint_time_left: i32,
    /// Are rain tyres equipped
    pub rain_tyres: i32,
    ///
    pub session_index: i32,
    /// Used fuel since last time refueling
    pub used_fuel: f32,
    /// Delta time in wide character
    pub delta_lap_time: [u16; 15],
    /// Delta time time in milliseconds
    pub i_delta_lap_time: i32,
    /// Estimated lap time in milliseconds
    pub estimated_lap_time: [u16; 15],
    /// Estimated lap time in wide character
    pub i_estimated_lap_time: i32,
    /// Delta positive (1) or negative (0)
    pub is_delta_positive: i32,
    /// Last split time in milliseconds
    pub i_split: i32,
    /// Check if Lap is valid for timing
    pub is_valid_lap: i32,
    /// Laps possible with current fuel level
    pub fuel_estimated_laps: f32,
    /// Status of track
    pub track_status: [u16; 33],
    /// Mandatory pitstops the player still has to do
    pub missing_mandatory_pits: i32,
    /// Time of day in seconds
    pub clock: f32,
    /// Is Blinker left on
    pub direction_lights_left: i32,
    /// Is Blinker right on
    pub direction_lights_right: i32,
    /// Yellow Flag is out?
    pub global_yellow: i32,
    /// Yellow Flag in Sector 1 is out?
    pub global_yellow1: i32,
    /// Yellow Flag in Sector 2 is out?
    pub global_yellow2: i32,
    /// Yellow Flag in Sector 3 is out?
    pub global_yellow3: i32,
    /// White Flag is out?
    pub global_white: i32,
    /// Green Flag is out?
    pub global_green: i32,
    /// Checkered Flag is out?
    pub global_chequered: i32,
    /// Red Flag is out?
    pub global_red: i32,
    /// Number of tyre set on the MFD
    pub mfd_tyre_set: i32,
    /// How much fuel to add on the MFD
    pub mfd_fuel_to_add: f32,
    /// Tyre pressure left front on the MFD
    pub mfd_tyre_pressure_lf: f32,
    /// Tyre pressure right front on the MFD
    pub mfd_tyre_pressure_rf: f32,
    /// Tyre pressure left rear on the MFD
    pub mfd_tyre_pressure_lr: f32,
    /// Tyre pressure right rear on the MFD
    pub mfd_tyre_pressure_rr: f32,
    /// See enums ACC_TRACK_GRIP_STATUS
    pub track_grip_status: TrackGripStatusRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_10m: RainIntensityRaw,
    /// See enums ACC_RAIN_INTENSITY
    pub rain_intensity_in_30m: RainIntensityRaw,
    /// Tyre Set currently in use
    pub current_tyre_set: i32,
    /// Next Tyre set per strategy
    pub strategy_tyre_set: i32,
    /// Distance in ms to car in front
    pub gap_ahead: i32,
    /// Distance in ms to car behind
    pub gap_behind: i32,
}

/// Data that never changes during a session.
#[repr(C, packed(4))]
#[derive(Clone, Debug, TypeName)]
pub struct PageFileStatic {
    /// Shared memory version
    pub sm_version: [u16; 15],
    /// Assetto Corsa version
    pub ac_version: [u16; 15],
    /// Number of sessions
    pub number_of_sessions: i32,
    /// Number of cars
    pub num_cars: i32,
    /// Player car model see Appendix 2
    pub car_model: [u16; 33],
    /// Track name
    pub track: [u16; 33],
    /// Player name
    pub player_name: [u16; 33],
    /// Player surname
    pub player_surname: [u16; 33],
    /// Player nickname
    pub player_nick: [u16; 33],
    /// Number of sectors
    pub sector_count: i32,
    /// Not shown in ACC
    pub max_torque: f32,
    /// Not shown in ACC
    pub max_power: f32,
    /// Maximum rpm
    pub max_rpm: i32,
    /// Maximum fuel tank capacity
    pub max_fuel: f32,
    /// Not shown in ACC
    pub suspension_max_travel: [f32; 4],
    /// Not shown in ACC
    pub tyre_radius: [f32; 4],
    /// Maximum turbo boost
    pub max_turbo_boost: f32,
    ///  
    pub deprecated_1: f32,
    ///  
    pub deprecated_2: f32,
    /// Penalties enabled
    pub penalties_enabled: i32,
    /// Fuel consumption rate
    pub aid_fuel_rate: f32,
    /// Tyre wear rate
    pub aid_tyre_rate: f32,
    /// Mechanical damage rate
    pub aid_mechanical_damage: f32,
    /// Not allowed in Blancpain endurance series
    pub aid_allow_tyre_blankets: f32,
    /// Stability control used
    pub aid_stability: f32,
    /// Auto clutch used
    pub aid_auto_clutch: i32,
    /// Always true in ACC
    pub aid_auto_blip: i32,
    /// Not used in ACC
    pub has_drs: i32,
    /// Not used in ACC
    pub has_ers: i32,
    /// Not used in ACC
    pub has_kers: i32,
    /// Not used in ACC
    pub kers_max_j: f32,
    /// Not used in ACC
    pub engine_brake_settings_count: i32,
    /// Not used in ACC
    pub ers_power_controller_count: i32,
    /// Not used in ACC
    pub track_spline_length: f32,
    /// Not used in ACC
    pub track_configuration: [u16; 33],
    /// Not used in ACC
    pub ers_max_j: f32,
    /// Not used in ACC
    pub is_timed_race: i32,
    /// Not used in ACC
    pub has_extra_lap: i32,
    /// Not used in ACC
    pub car_skin: [u16; 33],
    /// Not used in ACC
    pub reversed_grid_positions: i32,
    /// Pit window opening time
    pub pit_window_start: i32,
    /// Pit windows closing time
    pub pit_window_end: i32,
    /// If is a multiplayer session
    pub is_online: i32,
    /// Name of the dry tyres
    pub dry_tyres_name: [u16; 33],
    /// Name of the wet tyres
    pub wet_tyres_name: [u16; 33],
}
