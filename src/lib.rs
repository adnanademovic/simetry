pub use racing_flags::RacingFlags;
use std::future::Future;
use std::time::Duration;
use tokio::select;
use uom::si::f64::{AngularVelocity, Velocity};

pub mod assetto_corsa;
pub mod assetto_corsa_competizione;
pub mod iracing;
mod racing_flags;
pub mod rfactor_2;
mod windows_util;

enum SimetrySource {
    IRacing(iracing::Client),
    AssettoCorsa(assetto_corsa::Client),
    AssettoCorsaCompetizione(assetto_corsa_competizione::Client),
    RFactor2(rfactor_2::Client),
}

pub struct Simetry {
    inner: SimetrySource,
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

impl Simetry {
    pub async fn connect() -> Self {
        let retry_delay = Duration::from_secs(5);
        let iracing_future = loop_until_success(iracing::Client::connect, retry_delay);
        let assetto_corsa_future = loop_until_success(assetto_corsa::Client::connect, retry_delay);
        let assetto_corsa_competizione_future =
            loop_until_success(assetto_corsa_competizione::Client::connect, retry_delay);
        let rfactor_2_future = rfactor_2::Client::connect();
        let inner = select! {
            x = iracing_future => SimetrySource::IRacing(x),
            x = assetto_corsa_future => SimetrySource::AssettoCorsa(x),
            x = assetto_corsa_competizione_future => SimetrySource::AssettoCorsaCompetizione(x),
            x = rfactor_2_future => SimetrySource::RFactor2(x),
        };
        Self { inner }
    }

    pub fn sim_name(&self) -> &str {
        match self.inner {
            SimetrySource::IRacing(_) => "iRacing",
            SimetrySource::AssettoCorsa(_) => "Assetto Corsa",
            SimetrySource::AssettoCorsaCompetizione(_) => "Assetto Corsa Competizione",
            SimetrySource::RFactor2(_) => "rFactor 2",
        }
    }

    pub async fn next_moment(&mut self) -> Option<Moment> {
        Some(Moment {
            inner: match &mut self.inner {
                SimetrySource::IRacing(v) => MomentSource::IRacing(v.next_sim_state().await?),
                SimetrySource::AssettoCorsa(v) => {
                    MomentSource::AssettoCorsa(v.next_sim_state().await?)
                }
                SimetrySource::AssettoCorsaCompetizione(v) => {
                    MomentSource::AssettoCorsaCompetizione(v.next_sim_state().await?)
                }
                SimetrySource::RFactor2(v) => MomentSource::RFactor2(v.next_sim_state().await?),
            },
        })
    }
}

pub async fn connect() -> Simetry {
    Simetry::connect().await
}

enum MomentSource {
    IRacing(iracing::SimState),
    AssettoCorsa(assetto_corsa::SimState),
    AssettoCorsaCompetizione(assetto_corsa_competizione::SimState),
    RFactor2(rfactor_2::SimState),
}

pub trait MomentImpl {
    fn car_left(&self) -> bool;
    fn car_right(&self) -> bool;
    fn basic_telemetry(&self) -> Option<BasicTelemetry>;
    fn shift_point(&self) -> Option<AngularVelocity>;
    fn flags(&self) -> RacingFlags;
    fn car_model_id(&self) -> Option<String>;
}

pub struct Moment {
    inner: MomentSource,
}

impl Moment {
    fn source(&self) -> &dyn MomentImpl {
        match &self.inner {
            MomentSource::IRacing(v) => v,
            MomentSource::AssettoCorsa(v) => v,
            MomentSource::AssettoCorsaCompetizione(v) => v,
            MomentSource::RFactor2(v) => v,
        }
    }
}

pub struct BasicTelemetry {
    pub gear: i8,
    pub speed: Velocity,
    pub engine_rotation_speed: AngularVelocity,
    pub max_engine_rotation_speed: AngularVelocity,
    pub pit_limiter_engaged: bool,
    pub in_pit_lane: bool,
}

impl MomentImpl for Moment {
    fn car_left(&self) -> bool {
        self.source().car_left()
    }

    fn car_right(&self) -> bool {
        self.source().car_right()
    }

    fn basic_telemetry(&self) -> Option<BasicTelemetry> {
        self.source().basic_telemetry()
    }

    fn shift_point(&self) -> Option<AngularVelocity> {
        self.source().shift_point()
    }

    fn flags(&self) -> RacingFlags {
        self.source().flags()
    }

    fn car_model_id(&self) -> Option<String> {
        self.source().car_model_id()
    }
}
