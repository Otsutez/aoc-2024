use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug)]
enum Tile {
    Empty,
    Obstruction,
    Visited,
}

#[derive(Debug)]
struct Guard {
    row: isize,
    col: isize,
    row_inc: isize,
    col_inc: isize,
    initial_position: (isize, isize),
}

impl Guard {
    fn turn(&mut self) {
        let mut row_inc = self.row_inc;
        let mut col_inc = self.col_inc;

        if row_inc == -1 && col_inc == 0 {
            row_inc = 0;
            col_inc = 1;
        } else if row_inc == 0 && col_inc == 1 {
            row_inc = 1;
            col_inc = 0;
        } else if row_inc == 1 && col_inc == 0 {
            row_inc = 0;
            col_inc = -1;
        } else {
            row_inc = -1;
            col_inc = 0;
        }

        self.row_inc = row_inc;
        self.col_inc = col_inc;
    }

    fn forward(&mut self) {
        self.row += self.row_inc;
        self.col += self.col_inc;
    }

    fn reset_position(&mut self) {
        self.row = self.initial_position.0;
        self.col = self.initial_position.1;
        self.row_inc = -1;
        self.col_inc = 0;
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    guard: Guard,
    length: usize,
    width: usize,
    visited: HashSet<(usize, usize)>,
}

impl Map {
    fn simulate_once(&mut self) {
        while self.guard_in_map() {
            if self.guard_can_move_forward() {
                self.mark_visited();
                self.guard.forward();
            } else {
                self.guard.turn();
            }
        }
    }

    fn simulate_loop(&mut self) -> bool {
        let mut visited_turn = HashSet::new();
        self.guard.reset_position();
        while self.guard_in_map() {
            if self.guard_can_move_forward() {
                self.guard.forward();
            } else {
                self.guard.turn();
                if visited_turn.insert((
                    self.guard.row,
                    self.guard.col,
                    self.guard.row_inc,
                    self.guard.col_inc,
                )) == false
                {
                    // Player take this turn before, therefore in loop
                    return true;
                }
            }
        }
        false
    }

    fn count_possible_obstruction(&mut self) -> usize {
        let mut count = 0;
        for (row, col) in self.visited.clone().iter() {
            // Add obstacle
            self.map[*row][*col] = Tile::Obstruction;
            self.guard.reset_position();
            if self.simulate_loop() {
                count += 1;
            }
            // Remove obstacle
            self.map[*row][*col] = Tile::Empty;
        }

        count
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
    }

    fn mark_visited(&mut self) {
        let row = self.guard.row as usize;
        let col = self.guard.col as usize;
        self.visited.insert((row, col));
    }

    fn in_map(&self, row: isize, col: isize) -> bool {
        if row >= 0 && (row as usize) < self.length {
            if col >= 0 && (col as usize) < self.width {
                return true;
            }
        }
        false
    }

    fn guard_in_map(&self) -> bool {
        self.in_map(self.guard.row, self.guard.col)
    }

    fn guard_can_move_forward(&self) -> bool {
        let next_row = self.guard.row + self.guard.row_inc;
        let next_col = self.guard.col + self.guard.col_inc;

        if self.in_map(next_row, next_col) {
            match self.map[next_row as usize][next_col as usize] {
                Tile::Obstruction => false,
                _ => true,
            }
        } else {
            true
        }
    }

    fn print_visited_map(&self) {
        println!("");
        for row in self.map.iter() {
            for tile in row {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Obstruction => print!("#"),
                    Tile::Visited => print!("X"),
                }
            }
            println!("");
        }
        println!("");
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        let mut guard = Guard {
            row_inc: -1,
            col_inc: 0,
            row: 0,
            col: 0,
            initial_position: (0, 0),
        };

        let map = s
            .lines()
            .map(|line| {
                let map_row = line
                    .chars()
                    .map(|c| {
                        let tile = match c {
                            '#' => Tile::Obstruction,
                            '^' => {
                                guard.row = row;
                                guard.col = col;
                                guard.initial_position = (row, col);
                                Tile::Empty
                            }
                            _ => Tile::Empty,
                        };
                        col += 1;
                        tile
                    })
                    .collect::<Vec<_>>();
                row += 1;
                col = 0;
                map_row
            })
            .collect::<Vec<_>>();

        let length = map.len();
        let width = map[0].len();

        Ok(Map {
            guard,
            map,
            length,
            width,
            visited: HashSet::new(),
        })
    }
}

fn main() {
    let content = fs::read_to_string("input/day6").expect("Read input file");
    let mut map = Map::from_str(&content).expect("Parsing map");

    // Part 1
    map.simulate_once();
    let count = map.count_visited();
    println!("Number of position visited: {count}");

    // Part 2
    let count = map.count_possible_obstruction();
    println!("Number of possible obstructions: {count}");
}
