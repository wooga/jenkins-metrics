use chrono::Duration as CDuration;
use chrono::{DateTime, Utc};
use log::*;
use std::mem;

pub mod cli;
pub mod report;

pub struct ReportSample {
    reports: Vec<report::Report>,
    sample_size: CDuration,
    curr_sample: i32,
    start_date: DateTime<Utc>,
}

impl ReportSample {
    pub fn new(reports: Vec<report::Report>, now: DateTime<Utc>, sample_size: CDuration) -> Self {
        if let Some(r) = reports.get(0) {
            let duration = now - r.time();
            let max_samples =
                ((duration / (sample_size.num_seconds() as i32)).num_seconds()) as i32;
            let start_date = now - (sample_size * (max_samples + 1));
            ReportSample {
                reports,
                sample_size,
                curr_sample: 1,
                start_date,
            }
        } else {
            ReportSample {
                reports,
                sample_size: CDuration::zero(),
                curr_sample: 0,
                start_date: Utc::now(),
            }
        }
    }
}

impl Iterator for ReportSample {
    type Item = Vec<report::Report>;
    fn next(&mut self) -> Option<Vec<report::Report>> {
        debug!("fetch report sample");
        if self.reports.is_empty() {
            debug!("reports is empty");
            return None;
        }

        let split_date = self.start_date + (self.sample_size * self.curr_sample);
        trace!("split_date: {}", split_date);
        let mut split_index = 0;

        for report in self.reports.iter() {
            if report.time() > split_date {
                trace!("found split index: {}", split_index);
                break;
            }
            split_index += 1;
        }

        let split = self.reports.split_off(split_index);
        let sample = mem::replace(&mut self.reports, split);
        self.curr_sample += 1;
        Some(sample)
    }
}
