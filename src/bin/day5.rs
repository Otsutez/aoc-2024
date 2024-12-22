use std::collections::HashMap;
use std::fs;

#[cfg(feature = "part1")]
fn main() {
    let content = fs::read_to_string("input/day5").expect("Open input file");

    // Get rules
    let mut rules = HashMap::new();
    let mut line_iter = content.lines();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        let (x, y) = line.split_once('|').expect("Split line will be valid");
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        rules
            .entry(x)
            .and_modify(|v: &mut Vec<_>| v.push(y))
            .or_insert(vec![y]);
    }

    // dbg!(&rules);

    // Check if update is in correct order and sum up middle number
    let mut sum = 0;
    let mut incorrect_sum = 0;
    while let Some(line) = line_iter.next() {
        let mut update: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut is_correct_order = true;
        let length = update.len();
        for i in 0..length {
            match rules.get(&update[i]) {
                Some(must_come_before) => {
                    for j in 0..i {
                        if must_come_before.contains(&update[j]) {
                            is_correct_order = false;
                            let elem = update.remove(i);
                            update.insert(j, elem);
                            break;
                        }
                    }
                }
                None => continue,
            }
        }

        if is_correct_order {
            sum += update[length / 2];
            // dbg!(update);
        } else {
            incorrect_sum += update[length / 2];
        }
    }

    println!("Sum of middle pages of correctly ordered update: {sum}");
    println!("Sum of middle pages of incorrectly ordered update: {incorrect_sum}");
}
