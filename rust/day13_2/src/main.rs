use std::{cmp::min, vec};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut patterns:Vec<Vec<Vec<char>>> = vec![];
    let mut patterns_t: Vec<Vec<Vec<char>>> = vec![];
    
    let mut pattern: Vec<Vec<char>> = vec![];
    let mut sum = 0;

    for line in input.lines() {        
        if line == String::from("") {
            patterns.push(pattern.clone());
            pattern = vec![];
        } else {
            pattern.push(line.chars().collect::<Vec<char>>());
        }
    }

    for pattern in &patterns {
        patterns_t.push(vec![]);
        for idx in 0..pattern[0].len() {
            patterns_t.last_mut().unwrap().push(vec![]);
            for jdx in 0..pattern.len() {
                patterns_t.last_mut().unwrap().last_mut().unwrap().push(pattern[jdx][idx]);    
            }
        }
    }

    for k in 0..patterns.len() {
        let mut org = 0;
        let pattern = &patterns[k];
        for idx in 0..pattern.len()-1 {
            if pattern[idx] == pattern[idx+1] {
                let tests = min(idx+1, pattern.len() - idx - 1);
                if (1..=tests).all(|x| pattern[idx+x] == pattern[idx+1-x]) {
                    org = 100*(idx + 1);
                    break;
                }
            }
        }
        
        let pattern = &patterns_t[k];
        for idx in 0..pattern.len()-1 {
            if pattern[idx] == pattern[idx+1] {
                let tests = min(idx+1, pattern.len() - idx - 1);
                if (1..=tests).all(|x| pattern[idx+x] == pattern[idx+1-x]) {
                    org = idx + 1;
                    break;
                }
            }
        }

        let mut new = 0;
        let mut pattern: Vec<Vec<char>>= patterns[k].clone();
        let mut pattern_t: Vec<Vec<char>> = vec![];

        'xy: for x in 0..pattern.len() {
            for y in 0..pattern[0].len() {
                pattern = patterns[k].clone();
                pattern_t = patterns_t[k].clone();
                
                if pattern[x][y] == '#' {
                    pattern[x][y] = '.';
                    pattern_t[y][x] = '.';
                } else {
                    pattern[x][y] = '#';
                    pattern_t[y][x] = '#';
                }

                for idx in 0..pattern.len()-1 {
                    if pattern[idx] == pattern[idx+1] {
                        let tests = min(idx+1, pattern.len() - idx - 1);
                        if (1..=tests).all(|x| pattern[idx+x] == pattern[idx+1-x]) {
                            new = 100*(idx + 1);
                            if new != org {
                                break 'xy;
                            } else {
                                new = 0;
                            }
                        }
                    }
                }
                // if new != org && new != 0 {
                //     break 'xy;
                // }
                
                for idx in 0..pattern_t.len()-1 {
                    if pattern_t[idx] == pattern_t[idx+1] {
                        let tests = min(idx+1, pattern_t.len() - idx - 1);
                        if (1..=tests).all(|x| pattern_t[idx+x] == pattern_t[idx+1-x]) {
                            new = idx + 1;
                            if new != org {
                                break 'xy;
                            } else {
                                new = 0;
                            }
                        }
                    }
                }
                // if new != org && new != 0 {
                //     break 'xy;
                // }
            }
        }
        
        println!("{} {}", org, new);

        sum += new;

    } 

    println!("{:?}", sum);

}
