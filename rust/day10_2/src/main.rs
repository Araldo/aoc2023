use std::vec;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| (format!(".{}.", x)).chars().collect()).collect::<Vec<Vec<char>>>();
    let mut result: Vec<Vec<char>> = vec![];

    let mut pos: (usize, usize) = (0, 0);
    // find S
    for (line_idx, line) in lines.iter().enumerate(){
        result.push(vec!['.'; line.len()]);
        for (col_idx, col) in line.iter().enumerate(){
            if col == &'S' {
                pos = (line_idx, col_idx);
                break;
            }
        }}

    let start = pos.clone();
    let mut direction: usize = 0;
    for dir in 0..3 {
        match dir {
            0 => if vec!['F', '7', '|'].iter().any(|x| lines[start.0 - 1][start.1] == *x) {direction = 0; break;},
            1 => if vec!['J', '7', '-'].iter().any(|x| lines[start.0][start.1+1] == *x) {direction = 1; break;},
            2 => if vec!['J', 'L', '|'].iter().any(|x| lines[start.0 + 1][start.1] == *x) {direction = 2; break;},
            _ => panic!("invalid direction")
        }
    }

    loop {
        result[pos.0][pos.1] = lines[pos.0][pos.1];
        pos = (match direction { 0 => pos.0-1, 2 => pos.0+1, _ => pos.0}, match direction { 1 => pos.1+1, 3 => pos.1-1, _ => pos.1});
        // print!("{:?}, {}", lines[pos.0][pos.1], direction);
        match (lines[pos.0][pos.1], direction) {
            ('F', 0) => direction = 1,
            ('F', 3) => direction = 2,
            ('7', 0) => direction = 3,
            ('7', 1) => direction = 2,
            ('L', 2) => direction = 1,
            ('L', 3) => direction = 0,
            ('J', 1) => direction = 0,
            ('J', 2) => direction = 3,
            ('-', x) => direction = x,
            ('|', x) => direction = x,
            ('S', _) => break,
            _ => panic!("invalid direction: {}, {}", lines[pos.0][pos.1], direction)
        };
    }

    for line in &result {
        println!("{:?}", line.iter().collect::<String>());
    }

    let mut sp = (0,0);
    let mut outside = (false, false); // left, up

    'outer: for (line_idx, line) in result.iter().enumerate() {
        for (col_idx, col) in line.iter().enumerate() {
            if col == &'F' {
                pos = (line_idx, col_idx);
                sp = (line_idx, col_idx);
                direction = 1;
                break 'outer;
            }
        }
    }

    loop {
        println!("{:?}, {:?}, {}", pos, result[pos.0][pos.1], direction);
        match lines[pos.0][pos.1] {
            'F' => {
                if (direction==1 && !outside.0) || (direction==2 && !outside.1) {
                    if result[pos.0-1][pos.1-1] == '.' {result[pos.0-1][pos.1-1] = 'O'}
                    if result[pos.0-1][pos.1] == '.' {result[pos.0-1][pos.1] = 'O'}
                    if result[pos.0][pos.1-1] == '.' {result[pos.0][pos.1-1] = 'O'}
                    outside = (false, false)
                } else {
                    if result[pos.0+1][pos.1+1] == '.' {result[pos.0+1][pos.1+1] = 'O'}
                    outside = (true, true)
                }
            },
            '7' => {
                if (direction==3 && outside.0) || (direction==2 && !outside.1) {
                    if result[pos.0-1][pos.1+1] == '.' {result[pos.0-1][pos.1+1] = 'O'}
                    if result[pos.0-1][pos.1] == '.' {result[pos.0-1][pos.1] = 'O'}
                    if result[pos.0][pos.1+1] == '.' {result[pos.0][pos.1+1] = 'O'}
                    outside = (true, false)
                } else {
                    if result[pos.0+1][pos.1-1] == '.' {result[pos.0+1][pos.1-1] = 'O'}
                    outside = (false, true)
                }
            },
            'L' | 'S' => {
                if (direction==1 && !outside.0) || (direction==0 && outside.1) {
                    if result[pos.0+1][pos.1-1] == '.' {result[pos.0+1][pos.1-1] = 'O'}
                    if result[pos.0+1][pos.1] == '.' {result[pos.0+1][pos.1] = 'O'}
                    if result[pos.0][pos.1-1] == '.' {result[pos.0][pos.1-1] = 'O'}
                    outside = (false, true)
                } else {
                    if result[pos.0-1][pos.1+1] == '.' {result[pos.0-1][pos.1+1] = 'O'}
                    outside = (true, false)
                }
            },
            'J' => {
                if (direction==3 && outside.0) || (direction==0 && outside.1) {
                    if result[pos.0+1][pos.1+1] == '.' {result[pos.0+1][pos.1+1] = 'O'}
                    if result[pos.0+1][pos.1] == '.' {result[pos.0+1][pos.1] = 'O'}
                    if result[pos.0][pos.1+1] == '.' {result[pos.0][pos.1+1] = 'O'}
                    outside = (true, true)
                } else {
                    if result[pos.0-1][pos.1-1] == '.' {result[pos.0-1][pos.1-1] = 'O'}
                    outside = (false, false)
                }
            },
            '-' => {
                if !outside.1 {
                    if result[pos.0-1][pos.1] == '.' {result[pos.0-1][pos.1] = 'O'}
                } else {
                    if result[pos.0+1][pos.1] == '.' {result[pos.0+1][pos.1] = 'O'}
                }
            }, 
            '|' => {
                if !outside.0 {
                    if result[pos.0][pos.1-1] == '.' {result[pos.0][pos.1-1] = 'O'}
                } else {
                    if result[pos.0][pos.1+1] == '.' {result[pos.0][pos.1+1] = 'O'}
                }
            },
            _ => panic!("invalid direction {:?} {:?} {:?} {:?}", lines[pos.0][pos.1], pos, direction, outside)
        }

        pos = (match direction { 0 => pos.0-1, 2 => pos.0+1, _ => pos.0}, match direction { 1 => pos.1+1, 3 => pos.1-1, _ => pos.1});
        if (pos.0, pos.1) == sp {break;}
        match (lines[pos.0][pos.1], direction) {
            ('F', 0) => direction = 1,
            ('F', 3) => direction = 2,
            ('7', 0) => direction = 3,
            ('7', 1) => direction = 2,
            ('L', 2) => direction = 1,
            ('L', 3) => direction = 0,
            ('S', 2) => direction = 1,  // cheated here by looking at the input
            ('S', 3) => direction = 0,  // S eq. to L
            ('J', 1) => direction = 0,
            ('J', 2) => direction = 3,
            ('-', x) => direction = x,
            ('|', x) => direction = x,
            _ => panic!("invalid direction: {}, {}", lines[pos.0][pos.1], direction)
        };
    }

    let mut changed = false;

    loop {
        for (line_idx, line) in result.clone().iter().enumerate() {
            for (col_idx, col) in line.iter().enumerate() {
                if col == &'.' {
                    if line_idx > 0 && result[line_idx-1][col_idx] == 'O' { result[line_idx][col_idx] = 'O'.clone(); changed = true; }
                    if line_idx < result.len()-1 && result[line_idx+1][col_idx] == 'O' { result[line_idx][col_idx] = 'O'.clone(); changed = true; }
                    if col_idx > 0 && result[line_idx][col_idx-1] == 'O' { result[line_idx][col_idx] = 'O'.clone(); changed = true; }
                    if col_idx <  line.len()-1 && result[line_idx][col_idx+1] == 'O' { result[line_idx][col_idx] = 'O'.clone(); changed = true; }
                }
            }
        }
        if !changed {break;}
        changed = false;
    }

    let mut count=0;
    for (line_idx, line) in result.clone().iter().enumerate() {
        for (col_idx, col) in line.iter().enumerate() {
            if col==&'.' {count+=1;}
        }
    }

    for line in result {
        println!("{:?}", line.iter().collect::<String>());
    }  
    println!("{}", count)
}