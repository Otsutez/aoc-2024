use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32};
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::fs;

struct Mul {
    x: i32,
    y: i32,
}

#[cfg(feature = "part1")]
fn main() {
    let memory = fs::read_to_string("input/day3").expect("Read file");
    let (_, mul_pairs) = many1(extract_mul)(&memory).unwrap();
    let total = mul_pairs
        .into_iter()
        .fold(0, |acc, mul| acc + (mul.x * mul.y));

    println!("Results: {total}");
}

fn extract_mul(input: &str) -> IResult<&str, Mul> {
    let mul_command = delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"));
    let (input, (_, (x, y))) = many_till(anychar, mul_command)(input)?;
    Ok((input, Mul { x, y }))
}
