use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct QueuingDetails {
    #[serde(deserialize_with = "deserialize_duration_milis")]
    duration: Duration,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    blocked: Duration,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    waiting: Duration,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    buildable: Duration,
}

impl QueuingDetails {
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn blocked(&self) -> Duration {
        self.blocked
    }

    pub fn waiting(&self) -> Duration {
        self.waiting
    }

    pub fn buildable(&self) -> Duration {
        self.buildable
    }
}

fn deserialize_duration_milis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis: u64 = u64::deserialize(deserializer)?;
    let duration: Duration = Duration::from_millis(millis);
    Ok(duration)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    build: String,
    time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    duration: Duration,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    executing: Duration,
    executor_utilization: f64,
    queuing: QueuingDetails,
}

impl Report {

    pub fn build(&self) -> &str {
        &self.build
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn executing(&self) -> Duration {
        self.executing
    }

    pub fn queuing(&self) -> &QueuingDetails {
        &self.queuing
    }

    pub fn executor_utilization(&self) -> f64 {
        self.executor_utilization
    }
}
