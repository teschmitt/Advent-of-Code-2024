use anyhow::Result;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
};
use utils::get_input_file_as_string;
use utils::get_num;

fn is_safe(report: &Vec<u32>, skipped: bool, mut asc: Option<bool>) -> bool {
    // println!("Checking {report:?}");
    if report.len() <= 1 {
        return true;
    }

    let mut idx = 0;
    let r_len = report.len();
    while idx < r_len - 1 {
        let diff = report[idx + 1] as i32 - report[idx] as i32;

        if asc.is_some() && asc.unwrap() != (diff > 0) {
            if !skipped {
                let mut skipped_res = vec![];
                for i in idx.saturating_sub(1)..=(idx + 1) {
                    let mut skipped_report = report.clone();
                    skipped_report.remove(i);
                    skipped_res.push(is_safe(&skipped_report, true, None));
                }
                return skipped_res.iter().any(|&b| b);
            } else {
                return false;
            }
        }

        asc = Some(diff > 0);

        if diff.abs() == 0 || diff.abs() > 3 {
            if !skipped {
                let mut skipped_res = vec![];
                for i in idx.saturating_sub(1)..=(idx + 1) {
                    let mut skipped_report = report.clone();
                    skipped_report.remove(i);
                    skipped_res.push(is_safe(&skipped_report, true, asc));
                }
                return skipped_res.iter().any(|&b| b);
            } else {
                return false;
            }
        }

        idx += 1;
    }
    true
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
        let r_res = is_safe(report, false, None);
        // println!("{report:?} -> {r_res:?}");
        if r_res {
            res += 1;
            // dbg!(res);
        } else {
            println!("{report:?} -> {r_res:?}");
        }
    }

    dbg!(res);
    Ok(())
}
