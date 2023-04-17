pub use racing_flags::RacingFlags;
use std::future::Future;
use std::time::Duration;
use tokio::select;
use uom::si::f64::{AngularVelocity, Velocity};

pub mod assetto_corsa;
pub mod assetto_corsa_competizione;
pub mod dirt_rally_2;
pub mod iracing;
mod racing_flags;
pub mod rfactor_2;
mod windows_util;

#[async_trait::async_trait]
pub trait Simetry {
    fn name(&self) -> &str;

    async fn next_moment(&mut self) -> Option<Box<dyn Moment>>;
}

pub async fn connect_with_extra_options<F, R>(extra_options: F) -> Box<dyn Simetry>
where
    F: FnOnce() -> R,
    R: Future<Output = Box<dyn Simetry>>,
{
    select! {
        x = connect() => x,
        x = extra_options() => x,
    }
}

pub async fn connect() -> Box<dyn Simetry> {
    let retry_delay = Duration::from_secs(5);
    let iracing_future = loop_until_success(iracing::Client::connect, retry_delay);
    let assetto_corsa_future = loop_until_success(assetto_corsa::Client::connect, retry_delay);
    let assetto_corsa_competizione_future =
        loop_until_success(assetto_corsa_competizione::Client::connect, retry_delay);
    let rfactor_2_future = rfactor_2::Client::connect();
    select! {
        x = iracing_future => Box::new(x),
        x = assetto_corsa_future => Box::new(x),
        x = assetto_corsa_competizione_future => Box::new(x),
        x = rfactor_2_future => Box::new(x),
    }
}

async fn loop_until_success<R, T, F>(f: F, delay: Duration) -> R
where
    T: Future<Output = anyhow::Result<R>>,
    F: Fn() -> T,
{
    loop {
        if let Ok(v) = f().await {
            return v;
        }
        tokio::time::sleep(delay).await;
    }
}

pub trait Moment {
    fn car_left(&self) -> bool;
    fn car_right(&self) -> bool;
    fn basic_telemetry(&self) -> Option<BasicTelemetry>;
    fn shift_point(&self) -> Option<AngularVelocity>;
    fn flags(&self) -> RacingFlags;
    fn car_model_id(&self) -> Option<String>;
    fn ignition_on(&self) -> bool;
    fn starter_on(&self) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct BasicTelemetry {
    pub gear: i8,
    pub speed: Velocity,
    pub engine_rotation_speed: AngularVelocity,
    pub max_engine_rotation_speed: AngularVelocity,
    pub pit_limiter_engaged: bool,
    pub in_pit_lane: bool,
}
