use crate::rfactor_2::data::PacketId;
use crate::rfactor_2::shared_memory_data::{
    PageExtended, PageForceFeedback, PageHeader, PageMultiRules, PagePitInfo, PageRules,
    PageScoring, PageTelemetry, PageWeather,
};
use crate::rfactor_2::{
    Extended, ForceFeedback, MultiRules, PitInfo, Rules, Scoring, SimState, Telemetry, Weather,
};
use crate::windows_util::SharedMemory;
use std::sync::Arc;
use std::time::Duration;

// TODO: handle input files as well.

pub struct Client {
    sim_state_cache: SimState,

    /// Mapped view of rF2Telemetry structure
    telemetry: SharedMemory,
    /// Mapped view of rF2Scoring structure
    scoring: SharedMemory,
    /// Mapped view of rF2Rules structure
    rules: SharedMemory,
    /// Mapped view of rF2MultiRules structure
    multi_rules: SharedMemory,
    /// Mapped view of rF2ForceFeedback structure
    force_feedback: SharedMemory,
    /// Mapped view of rF2PitInfo structure
    pit_info: SharedMemory,
    /// Mapped view of rF2Weather structure
    weather: SharedMemory,
    /// Mapped view of rF2Extended structure
    extended: SharedMemory,
}

impl Client {
    pub async fn connect() -> Self {
        Self::connect_with_config(&Config {
            dedicated_server_pid: None,
            dedicated_server_global: false,
        })
        .await
    }

    pub async fn connect_with_config(config: &Config) -> Self {
        let poll_delay = Duration::from_millis(250);
        let telemetry = open_file("Telemetry", config, poll_delay).await;
        let scoring = open_file("Scoring", config, poll_delay).await;
        let rules = open_file("Rules", config, poll_delay).await;
        let multi_rules = open_file("MultiRules", config, poll_delay).await;
        let force_feedback = open_file("ForceFeedback", config, poll_delay).await;
        let pit_info = open_file("PitInfo", config, poll_delay).await;
        let weather = open_file("Weather", config, poll_delay).await;
        let extended = open_file("Extended", config, poll_delay).await;
        Self {
            sim_state_cache: SimState {
                telemetry: read_when_ready::<PageTelemetry, Telemetry>(&telemetry),
                scoring: read_when_ready::<PageScoring, Scoring>(&scoring),
                rules: read_when_ready::<PageRules, Rules>(&rules),
                multi_rules: read_when_ready::<PageMultiRules, MultiRules>(&multi_rules),
                force_feedback: read_when_ready::<PageForceFeedback, ForceFeedback>(
                    &force_feedback,
                ),
                pit_info: read_when_ready::<PagePitInfo, PitInfo>(&pit_info),
                weather: read_when_ready::<PageWeather, Weather>(&weather),
                extended: read_when_ready::<PageExtended, Extended>(&extended),
            },
            telemetry,
            scoring,
            rules,
            multi_rules,
            force_feedback,
            pit_info,
            weather,
            extended,
        }
    }

    pub fn force_feedback(&self) -> Arc<ForceFeedback> {
        read_when_ready::<PageForceFeedback, ForceFeedback>(&self.force_feedback)
    }

    pub async fn next_sim_state(&mut self) -> Option<SimState> {
        loop {
            let mut changed = false;

            if has_update_pending(self.sim_state_cache.telemetry.packet_id, &self.telemetry) {
                self.sim_state_cache.telemetry =
                    read_when_ready::<PageTelemetry, Telemetry>(&self.telemetry);
                changed = true;
            }
            if has_update_pending(self.sim_state_cache.scoring.packet_id, &self.scoring) {
                self.sim_state_cache.scoring =
                    read_when_ready::<PageScoring, Scoring>(&self.scoring);
                changed = true;
            }
            if has_update_pending(self.sim_state_cache.rules.packet_id, &self.rules) {
                self.sim_state_cache.rules = read_when_ready::<PageRules, Rules>(&self.rules);
                changed = true;
            }
            if has_update_pending(
                self.sim_state_cache.multi_rules.packet_id,
                &self.multi_rules,
            ) {
                self.sim_state_cache.multi_rules =
                    read_when_ready::<PageMultiRules, MultiRules>(&self.multi_rules);
                changed = true;
            }
            if has_update_pending(self.sim_state_cache.pit_info.packet_id, &self.pit_info) {
                self.sim_state_cache.pit_info =
                    read_when_ready::<PagePitInfo, PitInfo>(&self.pit_info);
                changed = true;
            }
            if has_update_pending(self.sim_state_cache.weather.packet_id, &self.weather) {
                self.sim_state_cache.weather =
                    read_when_ready::<PageWeather, Weather>(&self.weather);
                changed = true;
            }
            if has_update_pending(self.sim_state_cache.extended.packet_id, &self.extended) {
                self.sim_state_cache.extended =
                    read_when_ready::<PageExtended, Extended>(&self.extended);
                changed = true;
            }

            if changed {
                self.sim_state_cache.force_feedback =
                    read_when_ready::<PageForceFeedback, ForceFeedback>(&self.force_feedback);
                return Some(self.sim_state_cache.clone());
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
}

fn has_update_pending(old_id: PacketId, memory: &SharedMemory) -> bool {
    // If we were in the middle of a write, the ID will get updated by the next read.
    const RETURN_VALUE_IF_IN_MIDDLE_OF_UPDATE: bool = true;
    unsafe { memory.copy_as::<PageHeader>() }
        .try_into()
        .map_or(RETURN_VALUE_IF_IN_MIDDLE_OF_UPDATE, |v: PacketId| {
            v != old_id
        })
}

fn read_when_ready<Page: Copy, Data: TryFrom<Box<Page>>>(memory: &SharedMemory) -> Arc<Data> {
    loop {
        if let Ok(data) = Box::new(unsafe { *memory.get_as::<Page>() }).try_into() {
            return Arc::new(data);
        }
    }
}

pub struct Config {
    dedicated_server_pid: Option<String>,
    dedicated_server_global: bool,
}

async fn open_file(buffer_type: &str, config: &Config, poll_delay: Duration) -> SharedMemory {
    let name = format!(
        "{global}$rFactor2SMMP_{buffer_type}${pid}\0",
        global = if config.dedicated_server_global {
            "Global\\\\"
        } else {
            ""
        },
        pid = config.dedicated_server_pid.as_deref().unwrap_or(""),
    );
    SharedMemory::connect(name.as_bytes(), poll_delay).await
}
