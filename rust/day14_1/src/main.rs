fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    for _ in 0..lines.len() {
        let mut changed = false;
        for x in 0..lines.len() {
            if x == 0 {
                continue;
            }
            for y in 0..lines[x].len() {
                if lines[x][y] == 'O' && lines[x-1][y] == '.' {
                    lines[x-1][y] = 'O';
                    lines[x][y] = '.';
                    changed = true;
                }
            }
        }
        if changed == false {
            break;
        }   
    }

    let result = lines.iter().enumerate().map(
        |(idx, line)|
        line.iter().map(
            |x|
            if *x == 'O' { 1 } else { 0 }
        ).sum::<usize>() * (lines.len() - idx)
    ).sum::<usize>();

    println!("{:?}", result)
}
