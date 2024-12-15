use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many1, many_till},
    sequence::{delimited, tuple},
};
use utils::{get_input_file_as_string, get_u64};

fn main() -> Result<()> {
    let data = get_input_file_as_string()?;

    /* ---------------------------------------- parsers ---------------------------------------- */
    let get_mul = map(
        many_till(
            anychar,
            delimited(tag("mul("), tuple((get_u64, tag(","), get_u64)), tag(")")),
        ),
        |(_, (a, _, b))| a * b,
    );
    let mut get_all_muls = many1(get_mul);
    /* ----------------------------------------------------------------------------------------- */
    let res = get_all_muls(&data)
        .map_err(|err| anyhow!("nom error: {err:?}"))?
        .1
        .iter()
        .sum::<u64>();
    dbg!(res);

    drop(get_all_muls);
    Ok(())
}
