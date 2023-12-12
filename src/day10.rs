use std::str::FromStr;
use std::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;

pub fn main(data: &str) -> anyhow::Result<(u64, u64)> {
    //
    // part 1
    let map: Map = data.parse()?;
    let start = map.get_start_tile();
    let mut paths: HashMap<TileKey, Option<TileKey>> = [(start.key(), None)].into();
    let mut loops: Vec<HashSet<TileKey>> = Vec::new();
    let mut todo: VecDeque<_> = [start].into();
    while let Some(tile) = todo.pop_front() {
        for next in tile.neighbours() {
            match paths.entry(next.key()) {
                Entry::Vacant(e) => {
                    e.insert(Some(tile.key()));
                    todo.push_back(next);
                }
                Entry::Occupied(e) => {
                    if let Some(next_key) = e.get() {
                        if loops.iter().any(|p| p.contains(next_key)) {
                            continue
                        }
                        let mut path: HashSet<TileKey> = HashSet::new();
                        let mut todo: VecDeque<TileKey> = [*next_key, tile.key()].into();
                        while let Some(key) = todo.pop_front() {
                            path.insert(key);
                            if let Some(prev) = paths[&key] {
                                todo.push_back(prev);
                            }
                        }
                        assert!(path.len() & 1 == 1, "expected odd number of elements");
                        loops.push(path);
                    }
                }
            }
        }
    }

    let result1 = loops
        .iter()
        .map(|path| ((path.len() / 2) + 1) as u64)
        .max().expect("at least single loop exist");

    Ok((result1, 0))
}

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

impl Map {

    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }

    fn get_start_tile(&self) -> Tile {
        for (row, line) in self.0.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c == 'S' {
                    return Tile {
                        map: self,
                        from: Dir::Start,
                        c: 'S',
                        row,
                        col,
                    }
                }
            }
        }
        unreachable!("Start point must exist");
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.0[row][col]
    }

    fn iter_empty_spaces(&'_ self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.0
            .iter()
            .enumerate()
            .rev()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .rev()
                    .filter(|(_, c)| **c == '.')
                    .map(move |(col, _)| (row, col))
            })
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(raw: &str) -> anyhow::Result<Map> {
        let data = raw.lines().map(|l| l.trim().chars().collect()).collect();
        Ok(Map(data))
    }
}

type TileKey = (usize, usize);

#[derive(Clone)]
struct Tile<'map> {
    map: &'map Map,
    from: Dir,
    row: usize,
    col: usize,
    c: char,
}

impl<'map> Tile<'map> {
    fn key(&self) -> TileKey {
        (self.row, self.col)
    }

    fn neighbours(&self) -> impl Iterator<Item=Tile<'map>> {
        let n = [self.next_up(), self.next_right(), self.next_down(), self.next_left()];
        n.into_iter().filter_map(|t| t)
    }

    fn next_up(&self) -> Option<Tile<'map>> {
        if self.row == 0 || matches!(self.from, Dir::North) {
            return None
        }
        let row = self.row - 1;
        let c = self.map.get(row, self.col);
        match (self.c, c) {
            ('S' | '|' | 'L' |'J', '|' | '7' | 'F') | ('|' | 'L' |'J', 'S') => {
                Some(Tile {
                    map: self.map,
                    from: Dir::South,
                    col: self.col,
                    row,
                    c,
                })
            }
            _ => None
        }
    }
    fn next_right(&self) -> Option<Tile<'map>> {
        if self.col + 1 == self.map.width() || matches!(self.from, Dir::East) {
            return None
        }
        let col = self.col + 1;
        let c = self.map.get(self.row, col);
        match (self.c, c) {
            ('S' | '-' | 'L' | 'F', '-' | 'J' | '7') | ('-' | 'L' | 'F', 'S') => {
                Some(Tile {
                    map: self.map,
                    from: Dir::West,
                    row: self.row,
                    col,
                    c,
                })
            }
            _ => None
        }
    }
    fn next_down(&self) -> Option<Tile<'map>> {
        if self.row + 1 == self.map.height() || matches!(self.from, Dir::South) {
            return None
        }
        let row = self.row + 1;
        let c = self.map.get(row, self.col);
        match (self.c, c) {
            ('S' | '|' | 'F' |'7', '|' | 'L' | 'J') | ('|' | 'F' |'7', 'S') => {
                Some(Tile {
                    map: self.map,
                    from: Dir::North,
                    col: self.col,
                    row,
                    c,
                })
            }
            _ => None
        }
    }
    fn next_left(&self) -> Option<Tile<'map>> {
        if self.col == 0 || matches!(self.from, Dir::West) {
            return None
        }
        let col = self.col - 1;
        let c = self.map.get(self.row, col);
        match (self.c, c) {
            ('S' | '-' | 'J' | '7', '-' | 'F' | 'L') | ('-' | 'J' | '7', 'S') => {
                Some(Tile {
                    map: self.map,
                    from: Dir::East,
                    row: self.row,
                    col,
                    c,
                })
            }
            _ => None
        }
    }
}

impl<'map> fmt::Debug for Tile<'map> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tile")
            .field("from", &self.from)
            .field("row", &self.row)
            .field("col", &self.col)
            .field("c", &self.c)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Start,
    North,
    East,
    South,
    West,
}
