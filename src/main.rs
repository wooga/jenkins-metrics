use chrono::{DateTime, Utc};

use humantime::format_duration as f_duration;
use log::*;
use prettytable::{cell, row, Table};

use serde_json::from_reader;
use std::cmp::Ordering::Less;
use std::fs::File;
use std::io;
use std::iter::IntoIterator;
use std::path::Path;
use jenkins_metrics::cli::Options;
use jenkins_metrics::report::Report;
use jenkins_metrics::ReportSample;
use std::time::Duration;

const USAGE: &str = "
jenkins_metrics - Print basic jenkins build job metrics.

Usage:
  jenkins_metrics [options] [(--hours=D | --days=D | --weeks=D)] <metrics>
  jenkins_metrics (-h | --help)

Options:
  --filter=F                        simple prefix filter for build names
  --months=D                        data set range in months from today
  --weeks=D                         data set range in weeks from today
  --days=D                          data set range in days from today
  --sample-size=S                   Sub sample size in days
  -v, --verbose                     print more output
  -d, --debug                       print debug output
  --color WHEN                      Coloring: auto, always, never [default: auto]
  -h, --help                        show this help message and exit
";

fn load_reports<P: AsRef<Path>>(path:P) -> io::Result<Vec<Report>>{
    let json_file = File::open(path)?;
    let reports: Vec<Report> = from_reader(json_file)?;
    Ok(reports)
}

fn main() -> io::Result<()> {
    let options: Options = cli_core::get_options(USAGE)?;
    debug!("{:?}", options);

    let mut reports = load_reports(options.metrics())?;

    let now = Utc::now().date().and_hms(0, 0, 0);
    let min_date: DateTime<Utc> = now - options.duration();

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

    let builds_number = reports.len();
    info!("Metrics based on {} build records", builds_number);
    if let Some(f) = options.filter() {
        info!("filter builds start with: {}", f);
    }

    let sample_size = options.sample_size();
    if sample_size == options.duration() {
        trace!("calculate only one sample");
    } else {
        info!("sample size {}", f_duration(sample_size.to_std().unwrap()));
    }

    let mut overall_durations: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_executing: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_executor_utilization: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_queuing_durations: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_queuing_blocked: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_queuing_waiting: Vec<f64> = Vec::with_capacity(reports.len());
    let mut overall_queuing_buildable: Vec<f64> = Vec::with_capacity(reports.len());

    let mut durations: Vec<f64> = Vec::with_capacity(reports.len());
    let mut executing: Vec<f64> = Vec::with_capacity(reports.len());
    let mut executor_utilization: Vec<f64> = Vec::with_capacity(reports.len());
    let mut queuing_durations: Vec<f64> = Vec::with_capacity(reports.len());
    let mut queuing_blocked: Vec<f64> = Vec::with_capacity(reports.len());
    let mut queuing_waiting: Vec<f64> = Vec::with_capacity(reports.len());
    let mut queuing_buildable: Vec<f64> = Vec::with_capacity(reports.len());

    let iterator = ReportSample::new(reports, now, sample_size);
    let mut table = Table::new();

    table.add_row(row![
        "Sample",
        "Duration",
        "Duration (median)",
        "Executing",
        "Executing (median)",
        "Utilization",
        "Queuing",
        "Queuing (median)",
        "Q Blocked",
        "Q Waiting",
        "Q Buildable",
        "number of builds in sample"
    ]);

    for (i, reports) in iterator.enumerate() {
        if !reports.is_empty() {
            for report in &reports {
                durations.push(report.duration().as_secs_f64());
                executing.push(report.executing().as_secs_f64());
                executor_utilization.push(report.executor_utilization());
                queuing_durations.push(report.queuing().duration().as_secs_f64());
                queuing_blocked.push(report.queuing().blocked().as_secs_f64());
                queuing_waiting.push(report.queuing().waiting().as_secs_f64());
                queuing_buildable.push(report.queuing().buildable().as_secs_f64());
            }
            print_row(
                i,
                reports.len(),
                durations.as_mut_slice(),
                executing.as_mut_slice(),
                executor_utilization.as_mut_slice(),
                queuing_durations.as_mut_slice(),
                queuing_blocked.as_mut_slice(),
                queuing_waiting.as_mut_slice(),
                queuing_buildable.as_mut_slice(),
                &mut table,
            );
        } else {
            table.add_row(row![i, 0, 0, 0, 0, 0, 0, 0, reports.len()]);
        }

        //move all values to the overall vectors and clear at the same time.
        overall_durations.append(&mut durations);
        overall_executing.append(&mut executing);
        overall_executor_utilization.append(&mut executor_utilization);
        overall_queuing_durations.append(&mut queuing_durations);
        overall_queuing_blocked.append(&mut queuing_blocked);
        overall_queuing_waiting.append(&mut queuing_waiting);
        overall_queuing_buildable.append(&mut queuing_buildable);
    }

    if sample_size != options.duration() {
        table.add_row(row!["Overall",]);
        print_row(
            0,
            builds_number,
            overall_durations.as_mut_slice(),
            overall_executing.as_mut_slice(),
            overall_executor_utilization.as_mut_slice(),
            overall_queuing_durations.as_mut_slice(),
            overall_queuing_blocked.as_mut_slice(),
            overall_queuing_waiting.as_mut_slice(),
            overall_queuing_buildable.as_mut_slice(),
            &mut table,
        );
    }

    table.printstd();
    Ok(())
}

fn print_row(
    i: usize,
    len: usize,
    durations: &mut [f64],
    executing: &mut [f64],
    executor_utilization: &mut [f64],
    queuing_durations: &mut [f64],
    queuing_blocked: &mut [f64],
    queuing_waiting: &mut [f64],
    queuing_buildable: &mut [f64],
    table: &mut Table,
) {
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    executing.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    executor_utilization.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    queuing_durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    queuing_blocked.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    queuing_waiting.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
    queuing_buildable.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));

    table.add_row(row![
        i,
        f_duration(Duration::from_secs_f64(
            statistical::mean(&durations).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::median(&durations).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::mean(&executing).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::median(&executing).floor()
        )),
        format!("{:.2}", statistical::mean(&executor_utilization)),
        f_duration(Duration::from_secs_f64(
            statistical::mean(&queuing_durations).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::median(&queuing_durations).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::mean(&queuing_blocked).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::mean(&queuing_waiting).floor()
        )),
        f_duration(Duration::from_secs_f64(
            statistical::mean(&queuing_buildable).floor()
        )),
        len
    ]);
}
