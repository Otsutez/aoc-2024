use std::fs;

#[cfg(feature = "part1")]
fn main() {
    let input = fs::read_to_string("input/day2").expect("Read input file");

    let safe_report_count = input
        .lines()
        .map(|report| {
            let mut report = report.split_whitespace();
            let mut prev = report
                .next()
                .expect("report is not empty")
                .parse::<i32>()
                .unwrap();

            let mut diff_range = 0..=0;
            let mut set = false;

            while let Some(next) = report.next() {
                let next = next.parse::<i32>().unwrap();
                let diff = prev - next;

                if !set {
                    if diff.is_positive() {
                        diff_range = 1..=3;
                    } else {
                        diff_range = -3..=-1;
                    }
                    set = true;
                }

                if !diff_range.contains(&diff) {
                    return 0;
                }

                prev = next;
            }

            1
        })
        .fold(0, |count, x| count + x);

    println!("The number of safe reports are: {safe_report_count}");
}

// My failed attempt at solving part 2 by considering all the cases
#[cfg(feature = "part2")]
fn failed() {
    let input = fs::read_to_string("input/day2").expect("Read input file");

    let safe_report_count = input
        .lines()
        .map(|report| {
            let report: Vec<i32> = report
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();

            let mut diff_range = 0..=0;
            let mut set = false;
            let mut ignored = false;

            let mut prev_ptr = 0;
            let mut next_ptr = 1;

            while next_ptr < report.len() {
                let mut diff = report[prev_ptr] - report[next_ptr];

                if !set {
                    if diff.is_positive() {
                        diff_range = 1..=3;
                    } else {
                        diff_range = -3..=-1;
                    }
                    set = true;
                }

                if !diff_range.contains(&diff) {
                    // Check if skipping second value makes it valid
                    let skip_next = match report.get(next_ptr + 1) {
                        Some(val) => {
                            diff = report[prev_ptr] - val;
                            if diff_range.contains(&diff) {
                                if !ignored {
                                    next_ptr += 1;
                                    ignored = true;
                                } else {
                                    return 0;
                                }
                                true
                            } else if prev_ptr == 0 && (1..=3).contains(&diff.abs()) {
                                if !ignored {
                                    if diff.is_positive() {
                                        diff_range = 1..=3;
                                    } else {
                                        diff_range = -3..=-1;
                                    }
                                    next_ptr += 1;
                                    ignored = true;
                                } else {
                                    return 0;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        None => {
                            if !ignored {
                                true
                            } else {
                                false
                            }
                        }
                    };

                    // Check if skipping first value makes it valid
                    if !skip_next {
                        match prev_ptr {
                            0 => {
                                if !ignored {
                                    ignored = true;
                                } else {
                                    return 0;
                                }
                            }
                            _ => {
                                diff = report[prev_ptr - 1] - report[next_ptr];
                                if diff_range.contains(&diff) {
                                    if !ignored {
                                        ignored = true;
                                    } else {
                                        return 0;
                                    }
                                } else {
                                    return 0;
                                }
                            }
                        }
                    }
                }

                prev_ptr = next_ptr;
                next_ptr += 1;
            }

            1
        })
        .fold(0, |count, x| count + x);

    println!("The number of safe reports are: {safe_report_count}");
}

// My working solution that use bruteforcing and inefficient clone()
#[cfg(feature = "part2")]
fn main() {
    let input = fs::read_to_string("input/day2").expect("Read input file");

    let safe_report_count = input
        .lines()
        .map(|report| {
            let report: Vec<i32> = report
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();

            match get_unsafe_index(&report) {
                Some(index) => {
                    let mut safe = false;
                    // Bruteforce to check if it's safe
                    for i in 0..3 {
                        let index = match index.checked_sub(i) {
                            Some(index) => index,
                            None => break,
                        };
                        let v = create_skip_vec(&report, index);
                        match get_unsafe_index(&v) {
                            Some(_) => continue,
                            None => {
                                safe = true;
                                break;
                            }
                        }
                    }

                    if safe {
                        1
                    } else {
                        0
                    }
                }
                None => 1,
            }
        })
        .fold(0, |count, x| count + x);

    println!("The number of safe reports are: {safe_report_count}");
}

#[cfg(feature = "part2")]
fn get_unsafe_index(report: &Vec<i32>) -> Option<usize> {
    let mut diff_range = 0..=0;
    let mut set = false;

    let mut prev = 0;
    let mut next = 1;

    while next < report.len() {
        let diff = report[prev] - report[next];
        if !set {
            if diff.is_positive() {
                diff_range = 1..=3;
            } else {
                diff_range = -3..=-1;
            }
            set = true;
        }

        if !diff_range.contains(&diff) {
            return Some(next);
        }

        prev = next;
        next += 1
    }

    None
}

#[cfg(feature = "part2")]
fn create_skip_vec(v: &Vec<i32>, skip: usize) -> Vec<i32> {
    let mut new_v = Vec::new();
    v.iter().enumerate().for_each(|(i, val)| {
        if i != skip {
            new_v.push(val.clone());
        }
    });

    new_v
}
