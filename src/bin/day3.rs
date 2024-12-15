use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32};
use nom::combinator::map;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::fs;

#[cfg(feature = "part1")]
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

#[cfg(feature = "part1")]
fn extract_mul(input: &str) -> IResult<&str, Mul> {
    let mul_command = delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"));
    let (input, (_, (x, y))) = many_till(map(anychar, drop), mul_command)(input)?;
    Ok((input, Mul { x, y }))
}

#[cfg(feature = "part2")]
#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[cfg(feature = "part2")]
fn main() {
    let memory = fs::read_to_string("input/day3").expect("Read file");
    let (_, tokens) = many1(till_instruction)(&memory).unwrap();

    let mut enabled = true;
    let mut total = 0;
    for token in tokens {
        match token {
            Instruction::Mul(x, y) => {
                if enabled {
                    total += x * y;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }

    println!("Result: {total}");
}

#[cfg(feature = "part2")]
fn till_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, instruction)) =
        many_till(map(anychar, drop), alt((tag_do, tag_dont, tag_mul)))(input)?;
    Ok((input, instruction))
}

#[cfg(feature = "part2")]
fn tag_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, (x, y)) =
        delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"))(input)?;
    Ok((input, Instruction::Mul(x, y)))
}

#[cfg(feature = "part2")]
fn tag_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

#[cfg(feature = "part2")]
fn tag_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}
