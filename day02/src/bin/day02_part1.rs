#![feature(iter_map_windows)]

use anyhow::Result;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
};
use utils::{get_input_file_as_string, get_num};

type Report = Vec<u32>;

#[derive(Debug, PartialEq, Eq)]
enum ReportResult {
    Safe,
    Unsafe,
}

impl From<Report> for ReportResult {
    fn from(report: Report) -> Self {
        if report.len() <= 1 {
            return ReportResult::Safe;
        }
        if !(report.iter().is_sorted() || report.iter().rev().is_sorted()) {
            return ReportResult::Unsafe;
        }

        if report
            .iter()
            .map_windows(|[&x, &y]| 0 < x.abs_diff(y) && x.abs_diff(y) < 4)
            .any(|b| !b)
        {
            return ReportResult::Unsafe;
        }
        ReportResult::Safe
    }
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */
    let get_report = separated_list1(space1, get_num::<u32>);
    let mut get_all_reports = separated_list1(newline, get_report);
    /* ----------------------------------------------------------------------------------------- */

    let res = &get_all_reports(&data)
        .unwrap()
        .1
        .iter()
        .map(|r| r.clone().into())
        .filter(|res: &ReportResult| res == &ReportResult::Safe)
        .count();
    dbg!(res);
    Ok(())
}
