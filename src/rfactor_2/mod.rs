//! Support for rFactor 2.
//!
//! Requires installing and enabling plugin from <https://github.com/TheIronWolfModding/rF2SharedMemoryMapPlugin>.

mod client;
mod data;
mod shared_memory_data;

pub use client::{Client, Config};
pub use data::{Extended, ForceFeedback, MultiRules, PitInfo, Rules, Scoring, Telemetry, Weather};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SimState {
    pub telemetry: Arc<Telemetry>,
    pub scoring: Arc<Scoring>,
    pub rules: Arc<Rules>,
    pub multi_rules: Arc<MultiRules>,
    pub force_feedback: Arc<ForceFeedback>,
    pub pit_info: Arc<PitInfo>,
    pub weather: Arc<Weather>,
    pub extended: Arc<Extended>,
}
