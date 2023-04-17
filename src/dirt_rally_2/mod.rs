use crate::{unhandled, unhandled_default, BasicTelemetry, Moment, RacingFlags, Simetry};
use anyhow::Result;
use std::mem::transmute;
use tokio::net::UdpSocket;
use uom::si::angular_velocity::revolution_per_minute;
use uom::si::f64::{AngularVelocity, Velocity};
use uom::si::velocity::meter_per_second;

#[derive(Debug)]
pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub const DEFAULT_PORT: &'static str = "127.0.0.1:20777";

    #[inline]
    pub async fn connect_default() -> Result<Self> {
        Self::connect(Self::DEFAULT_PORT).await
    }

    pub async fn connect(port: &str) -> Result<Self> {
        let slf = Self {
            socket: UdpSocket::bind(port).await?,
        };
        slf.next_sim_state().await?;
        Ok(slf)
    }

    pub async fn next_sim_state(&self) -> Result<SimState> {
        let mut buffer = [0u8; PACKET_BUFFER_SIZE];
        let (_bytes, _origin) = self.socket.recv_from(&mut buffer).await?;
        Ok(unsafe { transmute(buffer) })
    }
}

const PACKET_BUFFER_SIZE: usize = 264;

#[repr(C, packed(4))]
#[derive(Clone, Debug, PartialEq)]
pub struct SimState {
    pub time: f32,
    pub time_of_current_lap: f32,
    pub distance_driven_on_current_lap: f32,
    pub distance_driven_overall: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub velocity_ms: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub roll_vector_x: f32,
    pub roll_vector_y: f32,
    pub roll_vector_z: f32,
    pub pitch_vector_x: f32,
    pub pitch_vector_y: f32,
    pub pitch_vector_z: f32,
    pub position_of_suspension_rear_left: f32,
    pub position_of_suspension_rear_right: f32,
    pub position_of_suspension_front_left: f32,
    pub position_of_suspension_front_right: f32,
    pub velocity_of_suspension_rear_left: f32,
    pub velocity_of_suspension_rear_right: f32,
    pub velocity_of_suspension_front_left: f32,
    pub velocity_of_suspension_front_right: f32,
    pub velocity_of_wheel_rear_left: f32,
    pub velocity_of_wheel_rear_right: f32,
    pub velocity_of_wheel_front_left: f32,
    pub velocity_of_wheel_front_right: f32,
    pub position_throttle: f32,
    pub position_steer: f32,
    pub position_brake: f32,
    pub position_clutch: f32,
    pub gear: f32,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub current_lap: f32,
    pub speed_of_engine_rpm_div_10: f32,
    unused1: [f32; 13],
    pub temperature_brake_rear_left: f32,
    pub temperature_brake_rear_right: f32,
    pub temperature_brake_front_left: f32,
    pub temperature_brake_front_right: f32,
    unused2: [f32; 5],
    pub number_of_laps_in_total: f32,
    pub length_of_track_in_total: f32,
    unused3: [f32; 1],
    pub maximum_rpm_div_10: f32,
    unused4: [u8; 8],
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "DirtRally2"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>> {
        Some(Box::new(self.next_sim_state().await.ok()?))
    }
}

impl Moment for SimState {
    fn car_left(&self) -> bool {
        unhandled(false)
    }

    fn car_right(&self) -> bool {
        unhandled(false)
    }

    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        let mut gear = self.gear as i8;
        if gear == 10 {
            gear = -1;
        }
        Some(BasicTelemetry {
            gear,
            speed: Velocity::new::<meter_per_second>(self.velocity_ms as f64),
            engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.speed_of_engine_rpm_div_10 as f64 * 10.0,
            ),
            max_engine_rotation_speed: AngularVelocity::new::<revolution_per_minute>(
                self.maximum_rpm_div_10 as f64 * 10.0,
            ),
            pit_limiter_engaged: unhandled(false),
            in_pit_lane: unhandled(false),
        })
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        unhandled(None)
    }

    fn flags(&self) -> RacingFlags {
        unhandled_default()
    }

    fn car_model_id(&self) -> Option<String> {
        unhandled(None)
    }

    fn ignition_on(&self) -> bool {
        unhandled(true)
    }

    fn starter_on(&self) -> bool {
        unhandled(false)
    }
}
