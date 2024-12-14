#![feature(iter_map_windows)]

use anyhow::Result;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
};
use utils::get_input_file_as_string;
use utils::get_num;

#[derive(Debug, PartialEq, Eq)]
enum ReportResult {
    Safe,
    Unsafe,
}

impl From<&Vec<u32>> for ReportResult {
    fn from(report: &Vec<u32>) -> Self {
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

    // let's be lazy and simply iterate over the index if the ReportResult is Unsafe
    let mut res = 0;
    for report in &get_all_reports(&data).unwrap().1 {
        let r_res: ReportResult = report.into();
        if r_res == ReportResult::Safe {
            res += 1;
            continue;
        } else {
            for idx in 0..report.len() {
                let mut red_report = report.clone();
                red_report.remove(idx);
                let r_res: ReportResult = (&red_report).into();
                if r_res == ReportResult::Safe {
                    res += 1;
                    break;
                }
            }
        }
    }

    dbg!(res);
    Ok(())
}
