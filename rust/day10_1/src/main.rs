fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut pos: (usize, usize) = (0, 0);
    // find S
    for (line_idx, line) in lines.iter().enumerate(){
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

    let mut steps = 1;
    loop {
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
        steps += 1;
    }

    print!("{:?}", steps/2);
}