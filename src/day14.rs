use std::str;
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::{Entry, DefaultHasher};
use std::hash::{Hash, Hasher};
use std::time::Instant;

const N: usize = 1_000_000_000; // _000_000;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {

    let mut grid: Grid = data.parse()?;
    grid.flip_north();
    let result1: usize = grid.north_weight();

    let mut grid: Grid = data.parse()?;
    let mut hm: HashMap<u64, usize> = HashMap::new();
    hm.insert(grid.get_hash(), 0);
    for i in 1..N {
        grid.flip_north();
        grid.flip_west();
        grid.flip_south();
        grid.flip_east();

        // find period
        match hm.entry(grid.get_hash()) {
            Entry::Vacant(v) => {
                v.insert(i);
            }
            Entry::Occupied(o) => {
                let period = i - o.get();
                let extra_rounds = (N - i) % period;
                for _ in 0..extra_rounds {
                    grid.flip_north();
                    grid.flip_west();
                    grid.flip_south();
                    grid.flip_east();
                }
                break
            }
        }
    }
    let result2 = grid.north_weight();

    Ok((result1, result2))
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Grid {
    data: Vec<Stone>,
    width: usize,
    height: usize,
}

impl Grid {
    fn flip_north(&mut self) {
        for c in 0..self.width {
            let mut dr = 0;
            for r in 0..self.height {
                let idx = r * self.width + c;
                match self.data[idx] {
                    Stone::None => {
                        dr += 1;
                    }
                    Stone::Square => {
                        dr = 0;
                    },
                    Stone::Round => {
                        let new_idx = (r - dr) * self.width + c;
                        self.data.swap(new_idx, idx);
                    },
                }
            }
        }
    }
    fn flip_west(&mut self) {
        for r in 0..self.height {
            let mut dc = 0;
            for c in 0..self.width {
                let idx = r * self.width + c;
                match self.data[idx] {
                    Stone::None => dc += 1,
                    Stone::Square => dc = 0,
                    Stone::Round => {
                        let new_idx = r * self.width + (c - dc);
                        self.data.swap(new_idx, idx);
                    }
                }
            }
        }
    }
    fn flip_south(&mut self) {
        for c in 0..self.width {
            let mut dr = 0;
            for r in (0..self.height).into_iter().rev() {
                let idx = r * self.width + c;
                match self.data[idx] {
                    Stone::None => {
                        dr += 1;
                    }
                    Stone::Square => {
                        dr = 0;
                    },
                    Stone::Round => {
                        let new_idx = (r + dr) * self.width + c;
                        self.data.swap(new_idx, idx);
                    },
                }
            }
        }
    }
    fn flip_east(&mut self) {
        for r in 0..self.height {
            let mut dc = 0;
            for c in (0..self.width).into_iter().rev() {
                let idx = r * self.width + c;
                match self.data[idx] {
                    Stone::None => dc += 1,
                    Stone::Square => dc = 0,
                    Stone::Round => {
                        let new_idx = r * self.width + (c + dc);
                        self.data.swap(new_idx, idx);
                    }
                }
            }
        }
    }

    fn north_weight(&self) -> usize {
        let mut result = 0;
        for (r, chunk) in self.data.chunks(self.width).enumerate() {
            result += chunk
                .iter()
                .filter(|s| matches!(s, Stone::Round))
                .count() * (self.height - r);
        }
        result
    }

    fn get_hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}

impl str::FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> anyhow::Result<Self> {
        let width = data.lines().next().expect("at least one line").len();
        let height = data.lines().count();
        let data = data
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Stone::None),
                '#' => Some(Stone::Square),
                'O' => Some(Stone::Round),
                '\n' => None,
                _ => unreachable!(),
            })
            .collect();
        Ok(Grid { data, width, height })
    }
}


impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for (i, s) in self.data.iter().enumerate() {
            if !first && i % self.width == 0 {
                f.write_str("\n")?;
            }
            first = false;
            match s {
                Stone::None => f.write_str(".")?,
                Stone::Round => f.write_str("O")?,
                Stone::Square => f.write_str("#")?,
            }
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stone {
    None,
    Square,
    Round,
}
