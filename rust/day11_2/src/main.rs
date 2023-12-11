use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();

    let line_len = lines[0].len();
    let col_len = lines.len();
    let empty_row = vec!['.'; line_len];
    let mut emtpy_rows: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<(usize, usize)> = vec![];

    let mut rows: Vec<Vec<char>> = vec![];
    for (line_idx, line) in lines.iter().enumerate() {
        if line == &empty_row {
            emtpy_rows.insert(line_idx);
        }
        rows.push(line.clone());
    }

    println!("{:?}", emtpy_rows);

    let mut empty_cols: HashSet<usize> = HashSet::new();
    for col_idx in 0..line_len {
        if (0..col_len).all(|x| &lines[x][col_idx] == &'.') {
            empty_cols.insert(col_idx);
        }
    }

    println!("{:?}", empty_cols);

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
            sum += (galaxy.0 as i64 - galaxies[jdx].0 as i64).abs() + ((galaxy.1 as i64 - galaxies[jdx].1 as i64)).abs();

            let lines_between = (galaxy.0..=galaxies[jdx].0).collect::<HashSet<usize>>();
            let extra_lines = lines_between.intersection(&emtpy_rows).collect::<Vec<&usize>>().len() as i64 * 999_999;
            
            let cols_between = if galaxy.1 < galaxies[jdx].1 {
                (galaxy.1..=galaxies[jdx].1).collect::<HashSet<usize>>()
            } else {
                (galaxies[jdx].1..=galaxy.1).collect::<HashSet<usize>>()
            };
            let extra_cols = cols_between.intersection(&empty_cols).collect::<Vec<&usize>>().len() as i64 * 999_999;

            sum += extra_lines + extra_cols;
        }
    }

    println!("{}", sum);
}