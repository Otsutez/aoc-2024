use std::fs;
use std::iter::zip;

#[cfg(feature = "part1")]
fn main() {
    let contents = fs::read_to_string("input/day1").expect("Read input file");

    // Convert into tuple of vector
    let mut lists: (Vec<_>, Vec<_>) = contents
        .lines()
        .filter_map(|line| {
            let line = line.split_once("   ")?;
            Some((
                line.0.parse::<i32>().unwrap(),
                line.1.parse::<i32>().unwrap(),
            ))
        })
        .unzip();

    lists.0.sort();
    lists.1.sort();

    let total = zip(lists.0, lists.1)
        .map(|(x, y)| x.abs_diff(y))
        .fold(0, |total, x| total + x);

    println!("The total difference is: {total}");
}

#[cfg(feature = "part2")]
fn main() {
    use std::collections::HashMap;

    let contents = fs::read_to_string("input/day1").expect("Read input file");

    // Convert into tuple of vector
    let (left_list, right_list): (Vec<_>, Vec<_>) = contents
        .lines()
        .filter_map(|line| {
            let line = line.split_once("   ")?;
            Some((
                line.0.parse::<i32>().unwrap(),
                line.1.parse::<i32>().unwrap(),
            ))
        })
        .unzip();

    // Get count of right list
    let mut map = HashMap::new();
    for x in right_list {
        map.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }

    let total = left_list
        .into_iter()
        .map(|x| x * map.get(&x).unwrap_or(&0))
        .fold(0, |total, x| total + x);

    println!("The similarity score is: {total}");
}
