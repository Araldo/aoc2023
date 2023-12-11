// use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();

    let line_len = lines[0].len();
    let col_len = lines.len();
    let empty_row = vec!['.'; line_len];
    let mut galaxies: Vec<(usize, usize)> = vec![];

    let mut rows: Vec<Vec<char>> = vec![];
    for line in &lines {
        if line == &empty_row {
            rows.push(line.clone());
        }
        rows.push(line.clone());
    }

    let mut empty_cols: Vec<usize> = vec![];
    for col_idx in 0..line_len {
        if (0..col_len).all(|x| &lines[x][col_idx] == &'.') {
            empty_cols.push(col_idx);
        }
    }

    for row in rows.iter_mut() {
        for col_idx in empty_cols.iter().rev() {
            row.insert(*col_idx, '.');
        }
    }

    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if col == &'#' {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut sum = 0;
    for (idx, galaxy) in galaxies.iter().enumerate() {
        for jdx in idx+1..galaxies.len() {
            sum += (galaxy.0 as i32 - galaxies[jdx].0 as i32).abs() + ((galaxy.1 as i32 - galaxies[jdx].1 as i32)).abs();
        }
    }

    println!("{}", sum);
}