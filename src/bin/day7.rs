use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::{newline, u64};
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use std::fs;

#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

#[cfg(feature = "part1")]
fn main() {
    let content = fs::read_to_string("input/day7").expect("Read file");
    let (_, equations) = parse_equations(&content).expect("Parse equations");
    let mut total = 0;
    for equation in equations {
        if can_be_true(&equation) {
            total += equation.test_value;
        }
    }

    println!("Total calibration result: {total}");
}

fn can_be_true(eqn: &Equation) -> bool {
    let operator_num = eqn.operands.len() - 1;
    let operator_set = (0..operator_num)
        .map(|_| vec!['+', '*'].into_iter())
        .multi_cartesian_product();

    for operators in operator_set {
        let mut result = eqn.operands[0];
        for i in 0..operators.len() {
            if operators[i] == '+' {
                result += eqn.operands[i + 1];
            } else {
                result *= eqn.operands[i + 1];
            }
        }

        if result == eqn.test_value {
            return true;
        }
    }

    false
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    many1(parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, test_value) = u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, operands) = parse_operands(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        Equation {
            test_value,
            operands,
        },
    ))
}

fn parse_operands(input: &str) -> IResult<&str, Vec<u64>> {
    many1(preceded(space0, u64))(input)
}
