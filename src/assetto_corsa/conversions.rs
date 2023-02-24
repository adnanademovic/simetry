use crate::assetto_corsa::data::{
    FlagType, Graphics, Penalty, Physics, SessionType, StaticData, Status,
};
use crate::assetto_corsa::shared_memory_data::{
    FlagTypeRaw, PageFileGraphics, PageFilePhysics, PageFileStatic, PenaltyRaw, SessionTypeRaw,
    StatusRaw,
};

pub fn extract_string(data: &[u16]) -> String {
    let length = data.iter().position(|v| *v == 0).unwrap_or(data.len());
    String::from_utf16(&data[0..length]).unwrap_or_default()
}

pub fn combine_car_info<T: Clone + Copy>(cars: [T; 60], items: usize) -> Vec<T> {
    cars.iter().take(items).copied().collect::<Vec<_>>()
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

impl From<PageFileGraphics> for Graphics {
    fn from(v: PageFileGraphics) -> Self {
        let active_cars = v.active_cars.max(0) as usize;

        Self {
            packet_id: v.packet_id,
            status: v.status.into(),
            session: v.session.into(),
            current_time: extract_string(&v.current_time),
            last_time: extract_string(&v.last_time),
            best_time: extract_string(&v.best_time),
            split: extract_string(&v.split),
            completed_laps: v.completed_laps,
            position: v.position,
            i_current_time: v.i_current_time,
            i_last_time: v.i_last_time,
            i_best_time: v.i_best_time,
            session_time_left: v.session_time_left,
            distance_traveled: v.distance_traveled,
            is_in_pit: v.is_in_pit,
            current_sector_index: v.current_sector_index,
            last_sector_time: v.last_sector_time,
            number_of_laps: v.number_of_laps,
            tyre_compound: extract_string(&v.tyre_compound),
            replay_time_multiplier: v.replay_time_multiplier,
            normalized_car_position: v.normalized_car_position,
            active_cars: v.active_cars,
            car_coordinates: combine_car_info(v.car_coordinates, active_cars),
            car_id: combine_car_info(v.car_id, active_cars),
            player_car_id: v.player_car_id,
            penalty_time: v.penalty_time,
            flag: v.flag.into(),
            penalty: v.penalty.into(),
            ideal_line_on: v.ideal_line_on,
            is_in_pit_lane: v.is_in_pit_lane,
            surface_grip: v.surface_grip,
            mandatory_pit_done: v.mandatory_pit_done,
            wind_speed: v.wind_speed,
            wind_direction: v.wind_direction,
            is_setup_menu_visible: v.is_setup_menu_visible,
            main_display_index: v.main_display_index,
            secondary_display_index: v.secondary_display_index,
            tc: v.tc,
            tc_cut: v.tc_cut,
            engine_map: v.engine_map,
            abs: v.abs,
            fuel_used_per_lap: v.fuel_used_per_lap,
            rain_lights: v.rain_lights,
            flashing_lights: v.flashing_lights,
            lights_stage: v.lights_stage,
            exhaust_temperature: v.exhaust_temperature,
            wiper_lv: v.wiper_lv,
            driver_stint_total_time_left: v.driver_stint_total_time_left,
            driver_stint_time_left: v.driver_stint_time_left,
            rain_tyres: v.rain_tyres,
            session_index: v.session_index,
            used_fuel: v.used_fuel,
            delta_lap_time: extract_string(&v.delta_lap_time),
            i_delta_lap_time: v.i_delta_lap_time,
            estimated_lap_time: extract_string(&v.estimated_lap_time),
            i_estimated_lap_time: v.i_estimated_lap_time,
            is_delta_positive: v.is_delta_positive,
            i_split: v.i_split,
            is_valid_lap: v.is_valid_lap,
            fuel_estimated_laps: v.fuel_estimated_laps,
            track_status: extract_string(&v.track_status),
            missing_mandatory_pits: v.missing_mandatory_pits,
            clock: v.clock,
            direction_lights_left: v.direction_lights_left,
            direction_lights_right: v.direction_lights_right,
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
            velocity: v.velocity,
            acc_g: v.acc_g,
            wheel_slip: v.wheel_slip,
            wheel_load: v.wheel_load,
            wheels_pressure: v.wheels_pressure,
            wheel_angular_speed: v.wheel_angular_speed,
            tyre_wear: v.tyre_wear,
            tyre_dirty_level: v.tyre_dirty_level,
            tyre_core_temperature: v.tyre_core_temperature,
            camber_rad: v.camber_rad,
            suspension_travel: v.suspension_travel,
            drs: v.drs,
            tc: v.tc,
            heading: v.heading,
            pitch: v.pitch,
            roll: v.roll,
            cg_height: v.cg_height,
            car_damage: v.car_damage,
            number_of_tyres_out: v.number_of_tyres_out,
            pit_limiter_on: v.pit_limiter_on,
            abs: v.abs,
            kers_charge: v.kers_charge,
            kers_input: v.kers_input,
            auto_shifter_on: v.auto_shifter_on,
            ride_height: v.ride_height,
            turbo_boost: v.turbo_boost,
            ballast: v.ballast,
            air_density: v.air_density,
            air_temp: v.air_temp,
            road_temp: v.road_temp,
            local_angular_vel: v.local_angular_vel,
            final_ff: v.final_ff,
            performance_meter: v.performance_meter,
            engine_brake: v.engine_brake,
            ers_recovery_level: v.ers_recovery_level,
            ers_power_level: v.ers_power_level,
            ers_heat_charging: v.ers_heat_charging,
            ers_is_charging: v.ers_is_charging,
            kers_current_kj: v.kers_current_kj,
            drs_available: v.drs_available,
            drs_enabled: v.drs_enabled,
            brake_temp: v.brake_temp,
            clutch: v.clutch,
            tyre_temp_i: v.tyre_temp_i,
            tyre_temp_m: v.tyre_temp_m,
            tyre_temp_o: v.tyre_temp_o,
            is_ai_controlled: v.is_ai_controlled,
            tyre_contact_point: v.tyre_contact_point,
            tyre_contact_normal: v.tyre_contact_normal,
            tyre_contact_heading: v.tyre_contact_heading,
            brake_bias: v.brake_bias,
            local_velocity: v.local_velocity,
            p2p_activations: v.p2p_activations,
            p2p_status: v.p2p_status,
            current_max_rpm: v.current_max_rpm,
            mz: v.mz,
            fx: v.fx,
            fy: v.fy,
            slip_ratio: v.slip_ratio,
            slip_angle: v.slip_angle,
            tc_in_action: v.tc_in_action,
            abs_in_action: v.abs_in_action,
            suspension_damage: v.suspension_damage,
            tyre_temp: v.tyre_temp,
            water_temp: v.water_temp,
            brake_pressure: v.brake_pressure,
            front_brake_compound: v.front_brake_compound,
            rear_brake_compound: v.rear_brake_compound,
            pad_life: v.pad_life,
            disc_life: v.disc_life,
            ignition_on: v.ignition_on,
            starter_engine_on: v.starter_engine_on,
            is_engine_running: v.is_engine_running,
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
            player_name: extract_string(&v.player_name),
            player_surname: extract_string(&v.player_surname),
            player_nick: extract_string(&v.player_nick),
            sector_count: v.sector_count,
            max_torque: v.max_torque,
            max_power: v.max_power,
            max_rpm: v.max_rpm,
            max_fuel: v.max_fuel,
            suspension_max_travel: v.suspension_max_travel,
            tyre_radius: v.tyre_radius,
            max_turbo_boost: v.max_turbo_boost,
            penalties_enabled: v.penalties_enabled,
            aid_fuel_rate: v.aid_fuel_rate,
            aid_tire_rate: v.aid_tire_rate,
            aid_mechanical_damage: v.aid_mechanical_damage,
            aid_allow_tyre_blankets: v.aid_allow_tyre_blankets,
            aid_stability: v.aid_stability,
            aid_auto_clutch: v.aid_auto_clutch,
            aid_auto_blip: v.aid_auto_blip,
            has_drs: v.has_drs,
            has_ers: v.has_ers,
            has_kers: v.has_kers,
            kers_max_j: v.kers_max_j,
            engine_brake_settings_count: v.engine_brake_settings_count,
            ers_power_controller_count: v.ers_power_controller_count,
            track_spline_length: v.track_spline_length,
            track_configuration: extract_string(&v.track_configuration),
            ers_max_j: v.ers_max_j,
            is_timed_race: v.is_timed_race,
            has_extra_lap: v.has_extra_lap,
            car_skin: extract_string(&v.car_skin),
            reversed_grid_positions: v.reversed_grid_positions,
            pit_window_start: v.pit_window_start,
            pit_window_end: v.pit_window_end,
            is_online: v.is_online,
        }
    }
}
