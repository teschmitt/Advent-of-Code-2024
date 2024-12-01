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
    }

    fn similarity_sum(&self) -> u64 {
        // to optimize for execution speed, build a loopup table of known counts and query that before filtering repeatedly
        self.left
            .iter()
            .map(|elem| elem * self.right.iter().filter(|&x| x == elem).count() as u64)
            .sum()
    }
}

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */
    let get_pair = separated_pair(get_u64, multispace1, get_u64);
    let mut get_all_pairs = separated_list1(newline, get_pair);
    /* ----------------------------------------------------------------------------------------- */

    let vec_of_pairs = get_all_pairs(&data.as_str()).unwrap().1;

    let cap = vec_of_pairs.len();
    let mut list_pair = LocationListPair {
        left: Vec::with_capacity(cap),
        right: Vec::with_capacity(cap),
    };
    list_pair.ingest(vec_of_pairs);

    dbg!(list_pair.similarity_sum());
    Ok(())
}
