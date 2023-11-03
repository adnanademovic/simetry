use crate::assetto_corsa_competizione::data::{Aids, CarDamage, GlobalFlags, Vector3, Wheels};
use crate::assetto_corsa_competizione::shared_memory_data::{
    FlagTypeRaw, PageFileGraphics, PageFilePhysics, PageFileStatic, PenaltyRaw, RainIntensityRaw,
    SessionTypeRaw, StatusRaw, TrackGripStatusRaw,
};
use crate::assetto_corsa_competizione::{
    FlagType, Graphics, LapTiming, MfdPitstop, Penalty, Physics, RainIntensity, SessionType,
    StaticData, Status, Time, TrackGripStatus, WheelInfo,
};

pub fn extract_string(data: &[u16]) -> String {
    let length = data.iter().position(|v| *v == 0).unwrap_or(data.len());
    String::from_utf16(&data[0..length]).unwrap_or_default()
}

impl From<PenaltyRaw> for Penalty {
    fn from(v: PenaltyRaw) -> Self {
        match v.data {
            0 => Penalty::None,
            1 => Penalty::DriveThroughCutting,
            2 => Penalty::StopAndGo10Cutting,
            3 => Penalty::StopAndGo20Cutting,
            4 => Penalty::StopAndGo30Cutting,
            5 => Penalty::DisqualifiedCutting,
            6 => Penalty::RemoveBestLaptimeCutting,
            7 => Penalty::DriveThroughPitSpeeding,
            8 => Penalty::StopAndGo10PitSpeeding,
            9 => Penalty::StopAndGo20PitSpeeding,
            10 => Penalty::StopAndGo30PitSpeeding,
            11 => Penalty::DisqualifiedPitSpeeding,
            12 => Penalty::RemoveBestLaptimePitSpeeding,
            13 => Penalty::DisqualifiedIgnoredMandatoryPit,
            14 => Penalty::PostRaceTime,
            15 => Penalty::DisqualifiedTrolling,
            16 => Penalty::DisqualifiedPitEntry,
            17 => Penalty::DisqualifiedPitExit,
            18 => Penalty::DisqualifiedWrongWay,
            19 => Penalty::DriveThroughIgnoredDriverStint,
            20 => Penalty::DisqualifiedIgnoredDriverStint,
            21 => Penalty::DisqualifiedExceededDriverStintLimit,
            _ => Penalty::None,
        }
    }
}

impl From<StatusRaw> for Status {
    fn from(v: StatusRaw) -> Self {
        match v.data {
            0 => Status::Off,
            1 => Status::Replay,
            2 => Status::Live,
            3 => Status::Pause,
            _ => Status::Off,
        }
    }
}

impl From<SessionTypeRaw> for SessionType {
    fn from(v: SessionTypeRaw) -> Self {
        match v.data {
            0 => SessionType::Practice,
            1 => SessionType::Qualify,
            2 => SessionType::Race,
            3 => SessionType::Hotlap,
            4 => SessionType::TimeAttack,
            5 => SessionType::Drift,
            6 => SessionType::Drag,
            7 => SessionType::HotStint,
            8 => SessionType::HotlapSuperPole,
            _ => SessionType::Unknown,
        }
    }
}

impl From<TrackGripStatusRaw> for TrackGripStatus {
    fn from(v: TrackGripStatusRaw) -> Self {
        match v.data {
            0 => TrackGripStatus::Green,
            1 => TrackGripStatus::Fast,
            2 => TrackGripStatus::Optimum,
            3 => TrackGripStatus::Greasy,
            4 => TrackGripStatus::Damp,
            5 => TrackGripStatus::Wet,
            6 => TrackGripStatus::Flooded,
            _ => TrackGripStatus::Green,
        }
    }
}

impl From<RainIntensityRaw> for RainIntensity {
    fn from(v: RainIntensityRaw) -> Self {
        match v.data {
            0 => RainIntensity::NoRain,
            1 => RainIntensity::Drizzle,
            2 => RainIntensity::LightRain,
            3 => RainIntensity::MediumRain,
            4 => RainIntensity::HeavyRain,
            5 => RainIntensity::Thunderstorm,
            _ => RainIntensity::NoRain,
        }
    }
}

impl From<FlagTypeRaw> for FlagType {
    fn from(v: FlagTypeRaw) -> Self {
        match v.data {
            0 => FlagType::None,
            1 => FlagType::Blue,
            2 => FlagType::Yellow,
            3 => FlagType::Black,
            4 => FlagType::White,
            5 => FlagType::Checkered,
            6 => FlagType::Penalty,
            7 => FlagType::Green,
            8 => FlagType::Orange,
            _ => FlagType::None,
        }
    }
}

impl<T> From<[T; 3]> for Vector3<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<[T; 4]> for Wheels<T> {
    fn from([front_left, front_right, rear_left, rear_right]: [T; 4]) -> Self {
        Self {
            front_left,
            front_right,
            rear_left,
            rear_right,
        }
    }
}

impl From<[f32; 5]> for CarDamage {
    fn from([front, rear, left, right, center]: [f32; 5]) -> Self {
        Self {
            front,
            rear,
            left,
            right,
            center,
        }
    }
}

fn time(text: [u16; 15], millis: i32) -> Time {
    Time {
        millis,
        text: extract_string(&text),
    }
}

fn combine_car_info<T: Clone + Copy>(cars: [T; 60], max_items: usize) -> Vec<T> {
    cars.iter().take(max_items).copied().collect::<Vec<_>>()
}

impl From<PageFileGraphics> for Graphics {
    fn from(v: PageFileGraphics) -> Self {
        let active_cars = v.active_cars.max(0) as usize;

        let car_coordinates_vec = combine_car_info(v.car_coordinates, active_cars);
        let car_id_vec = combine_car_info(v.car_id, active_cars);
        let car_coordinates = car_id_vec
            .into_iter()
            .zip(car_coordinates_vec)
            .map(|(id, coord)| (id, coord.into()))
            .collect();

        Self {
            packet_id: v.packet_id,
            status: v.status.into(),
            session: v.session.into(),
            lap_timing: LapTiming {
                current: time(v.current_time, v.i_current_time),
                last: time(v.last_time, v.i_last_time),
                best: time(v.best_time, v.i_best_time),
                split: time(v.split, v.i_split),
                delta_lap: time(v.delta_lap_time, v.i_delta_lap_time),
                estimated_lap: time(v.estimated_lap_time, v.i_estimated_lap_time),
                last_sector_ms: v.last_sector_time,
            },
            completed_laps: v.completed_laps,
            position: v.position,
            session_time_left: v.session_time_left,
            distance_traveled: v.distance_traveled,
            is_in_pit: v.is_in_pit != 0,
            current_sector_index: v.current_sector_index,
            tyre_compound: extract_string(&v.tyre_compound),
            normalized_car_position: v.normalized_car_position,
            car_coordinates,
            player_car_id: v.player_car_id,
            penalty_time: v.penalty_time,
            flag: v.flag.into(),
            penalty: v.penalty.into(),
            ideal_line_on: v.ideal_line_on != 0,
            is_in_pit_lane: v.is_in_pit_lane != 0,
            mandatory_pit_done: v.mandatory_pit_done != 0,
            wind_speed: v.wind_speed,
            wind_direction: v.wind_direction,
            is_setup_menu_visible: v.is_setup_menu_visible != 0,
            main_display_index: v.main_display_index,
            secondary_display_index: v.secondary_display_index,
            tc: v.tc,
            tc_cut: v.tc_cut,
            engine_map: v.engine_map,
            abs: v.abs,
            fuel_used_per_lap: v.fuel_used_per_lap,
            rain_lights: v.rain_lights != 0,
            flashing_lights: v.flashing_lights != 0,
            lights_stage: v.lights_stage,
            exhaust_temperature: v.exhaust_temperature,
            wiper_stage: v.wiper_lv,
            driver_stint_total_time_left: v.driver_stint_total_time_left,
            driver_stint_time_left: v.driver_stint_time_left,
            rain_tyres: v.rain_tyres != 0,
            session_index: v.session_index,
            used_fuel: v.used_fuel,
            is_delta_positive: v.is_delta_positive != 0,
            is_valid_lap: v.is_valid_lap != 0,
            fuel_estimated_laps: v.fuel_estimated_laps,
            track_status: extract_string(&v.track_status),
            missing_mandatory_pits: v.missing_mandatory_pits,
            clock: v.clock,
            direction_lights_left: v.direction_lights_left != 0,
            direction_lights_right: v.direction_lights_right != 0,
            global_flags: GlobalFlags {
                yellow: v.global_yellow != 0,
                yellow1: v.global_yellow1 != 0,
                yellow2: v.global_yellow2 != 0,
                yellow3: v.global_yellow3 != 0,
                white: v.global_white != 0,
                green: v.global_green != 0,
                chequered: v.global_chequered != 0,
                red: v.global_red != 0,
            },
            mfd_pitstop: MfdPitstop {
                tyre_set: v.mfd_tyre_set,
                fuel_to_add: v.mfd_fuel_to_add,
                tyre_pressures: Wheels {
                    front_left: v.mfd_tyre_pressure_lf,
                    front_right: v.mfd_tyre_pressure_rf,
                    rear_left: v.mfd_tyre_pressure_lr,
                    rear_right: v.mfd_tyre_pressure_rr,
                },
            },
            track_grip_status: v.track_grip_status.into(),
            rain_intensity: v.rain_intensity.into(),
            rain_intensity_in_30m: v.rain_intensity_in_30m.into(),
            rain_intensity_in_10m: v.rain_intensity_in_10m.into(),
            current_tyre_set: v.current_tyre_set,
            strategy_tyre_set: v.strategy_tyre_set,
        }
    }
}

impl From<PageFilePhysics> for Physics {
    fn from(v: PageFilePhysics) -> Self {
        Self {
            packet_id: v.packet_id,
            gas: v.gas,
            brake: v.brake,
            fuel: v.fuel,
            gear: v.gear,
            rpm: v.rpm,
            steer_angle: v.steer_angle,
            speed_kmh: v.speed_kmh,
            velocity: v.velocity.into(),
            acc_g: v.acc_g.into(),
            wheels: Wheels {
                front_left: WheelInfo {
                    tyre_pressure: v.wheels_pressure[0],
                    angular_speed: v.wheel_angular_speed[0],
                    suspension_travel: v.suspension_travel[0],
                    tyre_core_temperature: v.tyre_core_temperature[0],
                    brake_temperature: v.brake_temp[0],
                    tyre_contact_point: v.tyre_contact_point[0].into(),
                    tyre_contact_normal: v.tyre_contact_normal[0].into(),
                    tyre_contact_heading: v.tyre_contact_heading[0].into(),
                    slip: v.wheel_slip[0],
                    slip_ratio: v.slip_ratio[0],
                    slip_angle: v.slip_angle[0],
                    brake_pressure: v.brake_pressure[0],
                    pad_life: v.pad_life[0],
                    disc_life: v.disc_life[0],
                },
                front_right: WheelInfo {
                    tyre_pressure: v.wheels_pressure[1],
                    angular_speed: v.wheel_angular_speed[1],
                    suspension_travel: v.suspension_travel[1],
                    tyre_core_temperature: v.tyre_core_temperature[1],
                    brake_temperature: v.brake_temp[1],
                    tyre_contact_point: v.tyre_contact_point[1].into(),
                    tyre_contact_normal: v.tyre_contact_normal[1].into(),
                    tyre_contact_heading: v.tyre_contact_heading[1].into(),
                    slip: v.wheel_slip[1],
                    slip_ratio: v.slip_ratio[1],
                    slip_angle: v.slip_angle[1],
                    brake_pressure: v.brake_pressure[1],
                    pad_life: v.pad_life[1],
                    disc_life: v.disc_life[1],
                },
                rear_left: WheelInfo {
                    tyre_pressure: v.wheels_pressure[2],
                    angular_speed: v.wheel_angular_speed[2],
                    suspension_travel: v.suspension_travel[2],
                    tyre_core_temperature: v.tyre_core_temperature[2],
                    brake_temperature: v.brake_temp[2],
                    tyre_contact_point: v.tyre_contact_point[2].into(),
                    tyre_contact_normal: v.tyre_contact_normal[2].into(),
                    tyre_contact_heading: v.tyre_contact_heading[2].into(),
                    slip: v.wheel_slip[2],
                    slip_ratio: v.slip_ratio[2],
                    slip_angle: v.slip_angle[2],
                    brake_pressure: v.brake_pressure[2],
                    pad_life: v.pad_life[2],
                    disc_life: v.disc_life[2],
                },
                rear_right: WheelInfo {
                    tyre_pressure: v.wheels_pressure[3],
                    angular_speed: v.wheel_angular_speed[3],
                    suspension_travel: v.suspension_travel[3],
                    tyre_core_temperature: v.tyre_core_temperature[3],
                    brake_temperature: v.brake_temp[3],
                    tyre_contact_point: v.tyre_contact_point[3].into(),
                    tyre_contact_normal: v.tyre_contact_normal[3].into(),
                    tyre_contact_heading: v.tyre_contact_heading[3].into(),
                    slip: v.wheel_slip[3],
                    slip_ratio: v.slip_ratio[3],
                    slip_angle: v.slip_angle[3],
                    brake_pressure: v.brake_pressure[3],
                    pad_life: v.pad_life[3],
                    disc_life: v.disc_life[3],
                },
            },
            tc: v.tc,
            heading: v.heading,
            pitch: v.pitch,
            roll: v.roll,
            car_damage: v.car_damage.into(),
            pit_limiter_on: v.pit_limiter_on != 0,
            abs: v.abs,
            auto_shifter_on: v.auto_shifter_on != 0,
            turbo_boost: v.turbo_boost,
            air_temperature: v.air_temp,
            road_temperature: v.road_temp,
            local_angular_velocity: v.local_angular_vel.into(),
            final_ff: v.final_ff,
            clutch: v.clutch,
            is_ai_controlled: v.is_ai_controlled != 0,
            brake_bias: v.brake_bias,
            local_velocity: v.local_velocity.into(),
            water_temperature: v.water_temp,
            front_brake_compound: v.front_brake_compound,
            rear_brake_compound: v.rear_brake_compound,
            ignition_on: v.ignition_on != 0,
            starter_engine_on: v.starter_engine_on != 0,
            engine_running: v.is_engine_running != 0,
            kerb_vibration: v.kerb_vibration,
            slip_vibrations: v.slip_vibrations,
            g_vibrations: v.g_vibrations,
            abs_vibrations: v.abs_vibrations,
        }
    }
}

impl From<PageFileStatic> for StaticData {
    fn from(v: PageFileStatic) -> Self {
        Self {
            sm_version: extract_string(&v.sm_version),
            ac_version: extract_string(&v.ac_version),
            number_of_sessions: v.number_of_sessions,
            num_cars: v.num_cars,
            car_model: extract_string(&v.car_model),
            track: extract_string(&v.track),
            track_configuration: extract_string(&v.track_configuration),
            player_name: extract_string(&v.player_name),
            player_surname: extract_string(&v.player_surname),
            player_nick: extract_string(&v.player_nick),
            sector_count: v.sector_count,
            max_rpm: v.max_rpm,
            max_fuel: v.max_fuel,
            penalties_enabled: v.penalties_enabled,
            aids: Aids {
                fuel_rate: v.aid_fuel_rate,
                tyre_rate: v.aid_tyre_rate,
                mechanical_damage: v.aid_mechanical_damage,
                allow_tyre_blankets: v.aid_allow_tyre_blankets,
                stability: v.aid_stability,
                auto_clutch: v.aid_auto_clutch != 0,
                auto_blip: v.aid_auto_blip != 0,
            },
            pit_window_start: v.pit_window_start,
            pit_window_end: v.pit_window_end,
            is_online: v.is_online != 0,
            dry_tyres_name: extract_string(&v.dry_tyres_name),
            wet_tyres_name: extract_string(&v.wet_tyres_name),
        }
    }
}
