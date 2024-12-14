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

        let mut asc: Option<bool> = None;
        let mut skipped = false;
        let mut idx = 0;
        let mut next_idx = 1;
        while next_idx < report.len() {
            let diff = report[next_idx] as i32 - report[idx] as i32;
            if diff.abs() > 0 && diff.abs() < 4 {
                if asc.is_none() || ((diff > 0) == asc.unwrap()) {
                    asc = Some(diff > 0);
                    idx = next_idx;
                    next_idx += 1;
                    continue;
                }
                // change in direction
                else if !skipped {
                    skipped = true;
                    idx -= 1;
                    continue;
                } else {
                    return ReportResult::Unsafe;
                }
            }
            // two elements are equal
            else if !skipped {
                skipped = true;
                next_idx += 1;
                continue;
            } else {
                return ReportResult::Unsafe;
            }
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
        // println!("{report:?} -> {r_res:?}");
        if r_res == ReportResult::Safe {
            res += 1;
        }
    }

    dbg!(res);
    Ok(())
}
