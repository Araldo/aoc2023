use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut visited: HashMap<Vec<Vec<char>>, (i32, usize)> = HashMap::new();
    let mut results: Vec<i32> = vec![];
    let mut last_cycle = 0;

    for cycle in 1..=1000000000 {
        // north
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
        // west
        for _ in 0..lines.len() {
            let mut changed = false;
            for x in 0..lines.len() {
                for y in 0..lines[x].len() {
                    if y == 0 {
                        continue;
                    }
                    if lines[x][y] == 'O' && lines[x][y-1] == '.' {
                        lines[x][y-1] = 'O';
                        lines[x][y] = '.';
                        changed = true;
                    }
                }
            }
            if changed == false {
                break;
            }   
        }
        // south
        for _ in 0..lines.len() {
            let mut changed = false;
            for x in 0..lines.len()-1 {
                for y in 0..lines[x].len() {
                    if lines[x][y] == 'O' && lines[x+1][y] == '.' {
                        lines[x+1][y] = 'O';
                        lines[x][y] = '.';
                        changed = true;
                    }
                }
            }
            if changed == false {
                break;
            }   
        }
        // east
        for _ in 0..lines.len() {
            let mut changed = false;
            for x in 0..lines.len() {
                for y in 0..lines[x].len()-1 {
                    if lines[x][y] == 'O' && lines[x][y+1] == '.' {
                        lines[x][y+1] = 'O';
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
        results.push(result.try_into().unwrap());

        if visited.contains_key(&lines) {
            last_cycle = cycle.clone();
            break;
        } else {
            visited.insert(lines.clone(), (cycle, result.try_into().unwrap()));
        }
    }
    let start = visited[&lines].0;
    let index = (1000000000 - start) % (last_cycle - start) + start - 1;
    println!("{:?}", results[index as usize]);
}
