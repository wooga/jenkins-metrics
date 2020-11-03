use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use std::time::Duration;
use ordered_float::OrderedFloat;

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

#[derive(Debug, Deserialize, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    build: String,
    time: DateTime<Utc>,
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
}

impl PartialEq for Report {
    fn eq(&self, other: &Self) -> bool {
        self.build() == other.build()
    }
}
