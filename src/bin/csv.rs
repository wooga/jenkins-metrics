use chrono::Duration as CDuration;
use chrono::{DateTime, Utc};
use jenkins_metrics::cli::Options;
use jenkins_metrics::report::Report;
use log::*;
use prettytable::{cell, row, Table};
use serde_json::from_reader;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::path::Path;

const USAGE: &str = "
csv - Convert metrics json to csv.

Usage:
  csv [options] [(--days=D | --weeks=D | --hours=H)] [(--now | --today)] <metrics>...
  csv (-h | --help)

Options:
  --filter=F                        simple prefix filter for build names
  --weeks=D                         data set range in weeks from today
  --days=D                          data set range in days from today
  --hours=H                         data set range in hours from today
  --now                             Use current UTC time as base
  --today                           Include todays date
  -v, --verbose                     print more output
  -d, --debug                       print debug output
  --color WHEN                      Coloring: auto, always, never [default: auto]
  -h, --help                        show this help message and exit
";

fn load_reports<P: AsRef<Path>>(paths: &[P]) -> io::Result<Vec<Report>> {
    let mut final_reports = HashSet::new();
    for path in paths {
        let json_file = File::open(path)?;
        let reports: Vec<Report> = from_reader(json_file)?;
        for r in reports {
            final_reports.insert(r);
        }
    }

    Ok(final_reports.into_iter().collect())
}

fn main() -> io::Result<()> {
    let options: Options = cli_core::get_options(USAGE)?;
    debug!("{:?}", options);

    let mut reports = load_reports(options.metrics())?;

    let now = if options.now() {
        Utc::now()
    } else if options.today() {
        (Utc::now() + CDuration::days(1)).date().and_hms(0, 0, 0)
    } else {
        Utc::now().date().and_hms(0, 0, 0)
    };

    let min_date: DateTime<Utc> = now - options.duration();

    debug!("from: {:?} to: {:?}", min_date, now);

    reports
        .as_mut_slice()
        .sort_by(|a, b| a.time().partial_cmp(&b.time()).unwrap());

    let reports: Vec<Report> = reports
        .into_iter()
        .filter(|r| {
            if let Some(filter) = options.filter() {
                r.build().starts_with(filter)
            } else {
                true
            }
        })
        .filter(|r| r.time() > min_date && r.time() < now)
        .collect();

    let mut table = Table::new();
    table.set_titles(row!["build", "time", "Duration", "Executing", "Queueing"]);

    for report in reports {
        table.add_row(row![
            format!("{}", report.build()),
            format!("{}", report.time().format("%Y-%m-%d %H:%M:%S").to_string()),
            format!("{}", report.duration().as_secs()),
            format!("{}", report.executing().as_secs()),
            format!("{}", report.queuing().duration().as_secs()),
        ]);
    }

    table.to_csv(io::stdout())?;
    Ok(())
}
