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

#[cfg(feature = "part2")]
fn main() {}
