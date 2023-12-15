use std::str::FromStr;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {
    let mut result1 = 0;
    for chunk in data.split("\n\n") {
        let grid: Grid = chunk.parse()?;

        let mut col = 0;
        let it = grid
            .vertical_pairs()
            .enumerate()
            .filter(|(_, m)| matches!(m, Mirror::Exact));
        for (i, _) in it {
            let (left, right) = grid.vertical.split_at(i + 1);
            if left.iter().rev().zip(right).all(|(l, r)| l == r) {
                col = i + 1;
                break;
            }
        }

        let mut row = 0;
        let it = grid
            .horizontal_pairs()
            .enumerate()
            .filter(|(_, m)| matches!(m, Mirror::Exact));

        for (i, _) in it {
            let (left, right) = grid.horizontal.split_at(i + 1);
            if left.iter().rev().zip(right).all(|(l, r)| l == r) {
                row = 100 * (i + 1);
                break;
            }
        }
        result1 += col + row;
    }

    let mut result2 = 0;
    for chunk in data.split("\n\n") {
        let grid: Grid = chunk.parse()?;

        let mut col = 0;
        let it = grid
            .vertical_pairs()
            .enumerate()
            .filter(|(_, m)| matches!(m, Mirror::Exact | Mirror::Smudged));
        for (i, _) in it {
            let (left, right) = grid.vertical.split_at(i + 1);
            let mut fixed = false;
            let mut is_mirror = true;
            for mirror in left
                .iter()
                .rev()
                .zip(right)
                .map(|(a, b)| Mirror::from(*a, *b))
            {
                match mirror {
                    Mirror::Exact => (),
                    Mirror::Smudged => {
                        if fixed {
                            is_mirror = false;
                            break;
                        }
                        fixed = true
                    }
                    Mirror::None => {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if fixed && is_mirror {
                col = i + 1;
                break;
            }
        }

        let mut row = 0;
        let it = grid
            .horizontal_pairs()
            .enumerate()
            .filter(|(_, m)| matches!(m, Mirror::Exact | Mirror::Smudged));
        for (i, _) in it {
            let (left, right) = grid.horizontal.split_at(i + 1);
            let mut fixed = false;
            let mut is_mirror = true;
            for mirror in left
                .iter()
                .rev()
                .zip(right)
                .map(|(a, b)| Mirror::from(*a, *b))
            {
                match mirror {
                    Mirror::Exact => (),
                    Mirror::Smudged => {
                        if fixed {
                            is_mirror = false;
                            break;
                        }
                        fixed = true;
                    }
                    Mirror::None => {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if fixed && is_mirror {
                row = 100 * (i + 1);
                break;
            }
        }
        result2 += col + row;
    }

    Ok((result1, result2))
}

#[derive(Debug)]
struct Grid {
    vertical: Vec<usize>,
    horizontal: Vec<usize>,
}

impl Grid {
    fn vertical_pairs(&'_ self) -> impl Iterator<Item = Mirror> + '_ {
        self.vertical.windows(2).map(|i| Mirror::from(i[0], i[1]))
    }
    fn horizontal_pairs(&'_ self) -> impl Iterator<Item = Mirror> + '_ {
        self.horizontal.windows(2).map(|i| Mirror::from(i[0], i[1]))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> anyhow::Result<Self> {
        let w = data
            .lines()
            .next()
            .ok_or_else(|| anyhow::anyhow!("no lines"))?
            .len();
        let horizontal = data
            .lines()
            .map(|l| {
                l.chars().fold(0usize, |x, c| match c {
                    '#' => (x << 1) | 1,
                    '.' => x << 1,
                    _ => unreachable!("unreachable char: {}", c),
                })
            })
            .collect();
        let vertical = (0..w)
            .into_iter()
            .map(|rem| {
                data.chars()
                    .filter(|c| *c != '\n')
                    .enumerate()
                    .filter(|(i, _)| i % w == rem)
                    .fold(0usize, |x, (_, c)| match c {
                        '#' => (x << 1) | 1,
                        '.' => x << 1,
                        _ => unreachable!("unreachable char: {}", c),
                    })
            })
            .collect();
        Ok(Grid {
            horizontal,
            vertical,
        })
    }
}

#[derive(Debug)]
enum Mirror {
    None,
    Exact,
    Smudged,
}

impl Mirror {
    fn from(a: usize, b: usize) -> Self {
        if a == b {
            Mirror::Exact
        } else if (a ^ b).count_ones() == 1 {
            Mirror::Smudged
        } else {
            Mirror::None
        }
    }
}
