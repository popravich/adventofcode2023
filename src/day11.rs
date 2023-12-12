use std::collections::BTreeSet;
use std::cmp;

pub fn main(data: &str) -> anyhow::Result<(usize, usize)> {

    let map: Map = data.into();

    let pairs = map.galaxies.iter()
        .enumerate()
        .flat_map(|(i, pair1)| {
            map.galaxies
                .iter()
                .skip(i + 1)
                .map(move |pair2| (pair1, pair2))
        });
    let part2_pairs = pairs.clone();

    // part 1
    let mut result1 = 0;
    for ((row1, col1), (row2, col2)) in pairs {

        let row_lo = cmp::min(row1, row2);
        let row_hi = cmp::max(row1, row2);
        let doubles = map.empty_rows.range(row_lo..row_hi).count();
        let dr = row1.abs_diff(*row2) + doubles;

        let col_lo = cmp::min(col1, col2);
        let col_hi = cmp::max(col1, col2);
        let doubles = map.empty_cols.range(col_lo..col_hi).count();
        let dc = col1.abs_diff(*col2) + doubles;

        result1 += dr + dc;
    }

    // part 2
    let mut result2 = 0;
    for ((row1, col1), (row2, col2)) in part2_pairs {

        let row_lo = cmp::min(row1, row2);
        let row_hi = cmp::max(row1, row2);
        let doubles = map.empty_rows.range(row_lo..row_hi).count() * 999_999;
        let dr = row1.abs_diff(*row2) + doubles;

        let col_lo = cmp::min(col1, col2);
        let col_hi = cmp::max(col1, col2);
        let doubles = map.empty_cols.range(col_lo..=col_hi).count() * 999_999;
        let dc = col1.abs_diff(*col2) + doubles;

        result2 += dr + dc;
    }


    Ok((result1, result2))
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<(usize, usize)>,
    empty_rows: BTreeSet<usize>,
    empty_cols: BTreeSet<usize>,
}

impl From<&str> for Map {
    fn from(data: &str) -> Map {
        let mut lines = data.trim().lines();
        let columns = lines.next().expect("multiline input").trim().len();
        let rows = lines.count() + 1;

        let galaxies: Vec<(usize, usize)> = data
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(col, c)| (c == '#').then(|| (row, col)))
            }).collect();
        let empty_rows = (0..rows)
            .into_iter()
            .filter(|row| !galaxies.iter().any(|(r, _)| r == row))
            .collect();
        let empty_cols = (0..columns)
            .into_iter()
            .filter(|col| !galaxies.iter().any(|(_, c)| c == col))
            .collect();
        Map { galaxies, empty_rows, empty_cols }
    }
}
