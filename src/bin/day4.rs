use std::fs;

#[cfg(feature = "part1")]
fn main() {
    // Read into 2d vec
    let content = fs::read_to_string("input/day4").expect("Read input file");
    let word_search: Vec<&[u8]> = content.lines().map(|line| line.as_bytes()).collect();

    let mut count = 0;

    // indexed 1D array as 2D array
    let num_row = word_search.len();
    let num_col = word_search[0].len();
    let mas = "MAS".as_bytes();
    let amx = "AMX".as_bytes();

    // Search for XMAS
    for row in 0..num_row {
        let line = word_search[row];
        for col in 0..num_col {
            let char = line[col];
            if char == b'X' {
                count += count_xmas(row, col, &word_search, mas);
            } else if char == b'S' {
                count += count_xmas(row, col, &word_search, amx);
            }
        }
    }

    // count = count / 2;

    println!("XMAS count: {count}");
}

#[cfg(feature = "part1")]
fn count_xmas(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> usize {
    let mut count = 0;

    if check_hor(col, word_search[row], word) {
        count += 1;
    }
    if check_ver(row, col, word_search, word) {
        count += 1;
    }
    if check_neg_diag(row, col, word_search, word) {
        count += 1;
    }
    if check_pos_diag(row, col, word_search, word) {
        count += 1;
    }

    count
}

#[cfg(feature = "part1")]
fn check_hor(col: usize, line: &[u8], word: &[u8]) -> bool {
    if line.len() - col <= 3 {
        return false;
    }

    let mut is_same = true;
    for i in 1..=3 {
        if line[col + i] != word[i - 1] {
            is_same = false;
            break;
        }
    }
    is_same
}

#[cfg(feature = "part1")]
fn check_ver(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> bool {
    if word_search.len() - row <= 3 {
        return false;
    }

    let mut is_same = true;
    for i in 1..=3 {
        if word_search[row + i][col] != word[i - 1] {
            is_same = false;
            break;
        }
    }
    is_same
}

#[cfg(feature = "part1")]
fn check_neg_diag(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> bool {
    if word_search.len() - row <= 3 || word_search[row].len() - col <= 3 {
        return false;
    }

    let mut is_same = true;
    for i in 1..=3 {
        if word_search[row + i][col + i] != word[i - 1] {
            is_same = false;
            break;
        }
    }
    is_same
}

#[cfg(feature = "part1")]
fn check_pos_diag(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> bool {
    if row < 3 || word_search[row].len() - col <= 3 {
        return false;
    }

    let mut is_same = true;
    for i in 1..=3 {
        if word_search[row - i][col + i] != word[i - 1] {
            is_same = false;
            break;
        }
    }
    is_same
}

#[cfg(feature = "part2")]
fn main() {
    // Read into 2d vec
    let content = fs::read_to_string("input/day4").expect("Read input file");
    let word_search: Vec<&[u8]> = content.lines().map(|line| line.as_bytes()).collect();

    let mut count = 0;
    let num_row = word_search.len();
    let num_col = word_search[0].len();

    // Search for X-MAS
    for row in 1..num_row - 1 {
        let line = word_search[row];
        for col in 1..num_col - 1 {
            let char = line[col];
            if char == b'A' {
                if check_x_mas(row, col, &word_search) {
                    count += 1;
                }
            }
        }
    }

    println!("X-MAS count: {count}");
}

fn check_x_mas(row: usize, col: usize, word_search: &[&[u8]]) -> bool {
    let mas = "MAS".as_bytes();
    let sam = "SAM".as_bytes();

    // Check negative diagonal
    if check_neg_diag(row, col, word_search, mas) || check_neg_diag(row, col, word_search, sam) {
        // Check positive diagonal
        if check_pos_diag(row, col, word_search, mas) || check_pos_diag(row, col, word_search, sam)
        {
            return true;
        }
    }

    false
}

#[cfg(feature = "part2")]
fn check_neg_diag(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> bool {
    let start_row = row - 1;
    let start_col = col - 1;
    let mut is_same = true;
    for i in 0..3 {
        if word_search[start_row + i][start_col + i] != word[i] {
            is_same = false;
            break;
        }
    }
    is_same
}

#[cfg(feature = "part2")]
fn check_pos_diag(row: usize, col: usize, word_search: &[&[u8]], word: &[u8]) -> bool {
    let start_row = row + 1;
    let start_col = col - 1;
    let mut is_same = true;
    for i in 0..3 {
        if word_search[start_row - i][start_col + i] != word[i] {
            is_same = false;
            break;
        }
    }
    is_same
}
