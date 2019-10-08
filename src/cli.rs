use chrono::Duration as CDuration;
use cli_core::ColorOption;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Options {
    arg_metrics: PathBuf,
    flag_sample_size: Option<i64>,
    flag_filter: Option<String>,
    flag_weeks: Option<i64>,
    flag_days: Option<i64>,
    flag_verbose: bool,
    flag_debug: bool,
    flag_color: ColorOption,
}

impl Options {
    pub fn metrics(&self) -> &PathBuf {
        &self.arg_metrics
    }

    pub fn duration(&self) -> CDuration {
        if let Some(days) = self.flag_days {
            CDuration::days(days)
        } else if let Some(weeks) = self.flag_weeks {
            CDuration::weeks(weeks)
        } else {
            CDuration::weeks(8)
        }
    }

    pub fn sample_size(&self) -> CDuration {
        self.flag_sample_size.map(CDuration::days).unwrap_or_else(|| self.duration())
    }

    pub fn filter(&self) -> Option<&String> {
        self.flag_filter.as_ref()
    }
}

impl cli_core::Options for Options {
    fn verbose(&self) -> bool {
        self.flag_verbose
    }

    fn debug(&self) -> bool {
        self.flag_debug
    }

    fn color(&self) -> &ColorOption {
        &self.flag_color
    }
}
