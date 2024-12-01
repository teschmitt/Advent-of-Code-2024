use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

pub type Lines = io::Lines<BufReader<File>>;

// Get contents of the file provided in args[1] as Iterator
pub fn get_input_file() -> Result<Lines> {
    read_lines(get_path_from_args()?)
}

pub fn get_input_file_as_string() -> Result<String> {
    Ok(get_input_file()?
        .collect::<Result<Vec<String>, _>>()?
        .join("\n"))
}

fn read_lines<P>(filename: P) -> Result<Lines>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    assert!(file.metadata()?.is_file(), "Provided path is not a file");
    Ok(BufReader::new(file).lines())
}

pub fn get_path_from_args() -> Result<PathBuf> {
    Ok(PathBuf::from(
        env::args().nth(1).context("No argument passed")?,
    ))
}

pub fn get_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn get_num<NUM: FromStr>(input: &str) -> IResult<&str, NUM> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), NUM::from_str)(input)
}
