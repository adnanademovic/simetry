use typename::TypeName;

pub const MAX_MAPPED_VEHICLES: usize = 128;
pub const MAX_MAPPED_IDS: usize = 512;

type String64 = [u8; 64];
type String18 = [u8; 18];
type String16 = [u8; 16];
type Garbage = u8;

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, Default, TypeName)]
pub struct PageVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageHeader {
    /// Incremented right before buffer is written to.
    pub version_update_begin: u32,
    /// Incremented after buffer write is done.
    pub version_update_end: u32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageForceFeedback {
    pub ignored_header: PageHeader,

    /// Current FFB value reported via InternalsPlugin::ForceFeedback.
    pub force_value: f64,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
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
#[derive(Copy, Clone, Debug, TypeName)]
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
    expansion: [Garbage; 128 + 24],

    // keeping this at the end of the structure to make it easier to replace in future versions
    /// wheel info (front left, front right, rear left, rear right)    
    pub wheels: [PageWheelTelemetry; 4],
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
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
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageScoring {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageRules {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageMultiRules {
    pub header: PageHeader,

    /// How many bytes of the structure were written during the last update.
    ///
    /// 0 means unknown (whole buffer should be considered as updated).
    pub bytes_updated_hint: i32,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PagePitInfo {
    pub header: PageHeader,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageWeather {
    pub header: PageHeader,
}

#[repr(C, packed(4))]
#[derive(Copy, Clone, Debug, TypeName)]
pub struct PageExtended {
    pub header: PageHeader,
}
