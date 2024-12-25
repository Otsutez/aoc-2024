use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Antenna(isize, isize);

#[derive(Debug)]
enum Tile {
    Antenna(char),
    Empty,
    Antinode,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    antennas: HashMap<char, Vec<Antenna>>,
    width: isize,
    height: isize,
}

impl Map {
    fn print_map(&self) {
        for line in &self.map {
            for tile in line {
                match tile {
                    Tile::Antenna(c) => print!("{c}"),
                    Tile::Empty => print!("."),
                    Tile::Antinode => print!("#"),
                }
            }
            println!("");
        }
    }

    fn count_antinodes(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        self.antennas.clone().into_values().for_each(|v| {
            v.into_iter().tuple_combinations().for_each(|(x, y)| {
                let dy = x.0 - y.0;
                let dx = x.1 - y.1;
                let mut row = x.0 + dy;
                let mut col = x.1 + dx;
                if self.in_map(row, col) {
                    antinodes.insert((row, col));
                    self.map[row as usize][col as usize] = Tile::Antinode;
                }
                row = y.0 - dy;
                col = y.1 - dx;
                if self.in_map(row, col) {
                    antinodes.insert((row, col));
                    self.map[row as usize][col as usize] = Tile::Antinode;
                }
            })
        });
        antinodes.len()
    }

    fn count_antinodes_2(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        self.antennas.clone().into_values().for_each(|v| {
            v.into_iter().tuple_combinations().for_each(|(x, y)| {
                antinodes.insert((x.0, x.1));
                antinodes.insert((y.0, y.1));
                let dy = x.0 - y.0;
                let dx = x.1 - y.1;
                let mut row = x.0 + dy;
                let mut col = x.1 + dx;
                while self.in_map(row, col) {
                    antinodes.insert((row, col));
                    self.map[row as usize][col as usize] = Tile::Antinode;
                    row += dy;
                    col += dx;
                }
                row = y.0 - dy;
                col = y.1 - dx;
                while self.in_map(row, col) {
                    antinodes.insert((row, col));
                    self.map[row as usize][col as usize] = Tile::Antinode;
                    row -= dy;
                    col -= dx;
                }
            })
        });
        antinodes.len()
    }

    fn in_map(&self, row: isize, col: isize) -> bool {
        if (0..self.height).contains(&row) && (0..self.width).contains(&col) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let content = fs::read_to_string("input/day8").expect("Read input file");
    let mut map = parse_map(content);
    map.print_map();
    let count = map.count_antinodes();
    println!();
    map.print_map();
    println!("Number of antinodes part 1: {count}");
    let count = map.count_antinodes_2();
    println!();
    map.print_map();
    println!("Number of antinodes part 2: {count}");
}

fn parse_map(content: String) -> Map {
    let mut row = 0;
    let mut col = 0;
    let mut width = 0;
    let mut map = Vec::new();
    let mut line = Vec::new();
    let mut antennas = HashMap::new();
    content.chars().for_each(|c| match c {
        '.' => {
            line.push(Tile::Empty);
            col += 1;
        }
        '\n' => {
            map.push(std::mem::take(&mut line));
            width = col;
            row += 1;
            col = 0;
        }
        c => {
            line.push(Tile::Antenna(c));
            let mut antenna = vec![Antenna(row, col)];
            antennas
                .entry(c)
                .and_modify(|v: &mut Vec<Antenna>| v.append(&mut antenna))
                .or_insert(antenna);
            col += 1;
        }
    });
    Map {
        map,
        antennas,
        width,
        height: row,
    }
}
