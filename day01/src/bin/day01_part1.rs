use std::iter::zip;

use anyhow::Result;
use nom::{
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
};
use utils::{get_input_file_as_string, get_u64};

type LocationList = Vec<u64>;

#[derive(Debug, Clone)]
struct LocationListPair {
    left: LocationList,
    right: LocationList,
}

impl LocationListPair {
    fn ingest(&mut self, input: Vec<(u64, u64)>) {
        for pair in input {
            self.left.push(pair.0);
            self.right.push(pair.1);
        }
        self.left.sort_unstable();
        self.right.sort_unstable();
    }

    fn distance(&self) -> u64 {
        zip(self.left.clone(), self.right.clone())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;
    let mut list_pair = LocationListPair {
        left: vec![],
        right: vec![],
    };

    /* ---------------------------------------- parsers ---------------------------------------- */
    let get_pair = separated_pair(get_u64, multispace1, get_u64);
    let mut get_all_pairs = separated_list1(newline, get_pair);
    /* ----------------------------------------------------------------------------------------- */

    let vec_of_pairs = get_all_pairs(&data.as_str()).unwrap().1;

    list_pair.ingest(vec_of_pairs);
    dbg!(list_pair.distance());
    Ok(())
}
