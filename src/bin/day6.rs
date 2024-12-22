use std::{fs, str::FromStr};

#[derive(Debug)]
enum Tile {
    Empty,
    Obstruction,
    Visited,
}

#[derive(Debug)]
struct Player {
    row: isize,
    col: isize,
    row_inc: isize,
    col_inc: isize,
}

impl Player {
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
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    player: Player,
    length: usize,
    width: usize,
}

impl Map {
    fn simulate(&mut self) {
        while self.player_in_map() {
            if self.player_can_move_forward() {
                self.mark_visited();
                self.player.forward();
            } else {
                self.player.turn();
            }
        }
    }

    fn count_visited(&self) -> usize {
        let mut count = 0;
        for row in self.map.iter() {
            for tile in row {
                match tile {
                    Tile::Visited => {
                        count += 1;
                    }
                    _ => continue,
                }
            }
        }

        count
    }

    fn mark_visited(&mut self) {
        self.map[self.player.row as usize][self.player.col as usize] = Tile::Visited;
    }

    fn in_map(&self, row: isize, col: isize) -> bool {
        if row >= 0 && (row as usize) < self.length {
            if col >= 0 && (col as usize) < self.width {
                return true;
            }
        }
        false
    }

    fn player_in_map(&self) -> bool {
        self.in_map(self.player.row, self.player.col)
    }

    fn player_can_move_forward(&self) -> bool {
        let next_row = self.player.row + self.player.row_inc;
        let next_col = self.player.col + self.player.col_inc;

        if self.in_map(next_row, next_col) {
            match self.map[next_row as usize][next_col as usize] {
                Tile::Empty => true,
                Tile::Obstruction => false,
                Tile::Visited => true,
            }
        } else {
            true
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        let mut player = Player {
            row_inc: -1,
            col_inc: 0,
            row: 0,
            col: 0,
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
                                player.row = row;
                                player.col = col;
                                Tile::Visited
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
            player,
            map,
            length,
            width,
        })
    }
}

#[cfg(feature = "part1")]
fn main() {
    let content = fs::read_to_string("input/day6").expect("Read input file");
    let mut map = Map::from_str(&content).expect("Parsing map");
    map.simulate();
    let count = map.count_visited();
    println!("Number of position visited: {count}");
}
