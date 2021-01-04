use std::hash::{Hash, Hasher};
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Deserializer};
use std::time::Duration;

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
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

fn deserialize_ordered_float<'de, D>(deserializer: D) -> Result<OrderedFloat<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = f64::deserialize(deserializer)?;
    let ordered: OrderedFloat<f64> = OrderedFloat::from(value);
    Ok(ordered)
}

#[derive(Debug, Deserialize, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuildResult {
    Unknown,
    Aborted,
    Failure,
    NotBuild,
    Success,
    Unstable,
}

impl Default for BuildResult {
    fn default() -> Self {
        BuildResult::Unknown
    }
}

impl std::fmt::Display for BuildResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildResult::Unknown => write!(f, "unknown"),
            BuildResult::Aborted => write!(f, "aborted"),
            BuildResult::Failure => write!(f, "failure"),
            BuildResult::NotBuild => write!(f, "not_build"),
            BuildResult::Success => write!(f, "success"),
            BuildResult::Unstable => write!(f, "unstable"),
        }
    }
}

#[derive(Debug, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    build: String,
    time: DateTime<Utc>,
    #[serde(default)]
    result: BuildResult,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    duration: Duration,
    #[serde(deserialize_with = "deserialize_duration_milis")]
    executing: Duration,
    #[serde(deserialize_with = "deserialize_ordered_float")]
    executor_utilization: OrderedFloat<f64>,
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
        *self.executor_utilization
    }

    pub fn result(&self) -> &BuildResult {
        &self.result
    }
}

impl PartialEq for Report {
    fn eq(&self, other: &Self) -> bool {
        self.build() == other.build()
    }
}
impl Hash for Report {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.build.hash(state);
    }
}
    