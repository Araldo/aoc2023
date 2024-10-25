use std::cmp::min;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut patterns:Vec<Vec<Vec<char>>> = vec![];
    let mut patterns_t: Vec<Vec<Vec<char>>> = vec![];
    
    let mut pattern: Vec<Vec<char>> = vec![];

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

    let mut sum = 0;

    for pattern in &patterns {
        for idx in 0..pattern.len()-1 {
            if pattern[idx] == pattern[idx+1] {
                let tests = min(idx+1, pattern.len() - idx - 1);
                println!("{:?} {:?}", tests, idx);
                if (1..=tests).all(|x| pattern[idx+x] == pattern[idx+1-x]) {
                    sum += 100*(idx + 1);
                    break;
                }
            }
        }
    }

    for pattern in &patterns_t {
        for idx in 0..pattern.len()-1 {
            if pattern[idx] == pattern[idx+1] {
                let tests = min(idx+1, pattern.len() - idx - 1);
                if (1..=tests).all(|x| pattern[idx+x] == pattern[idx+1-x]) {
                    sum += idx + 1;
                    break;
                }
            }
        }
    }

    println!("{:?}", sum);
    // println!("{:?}", patterns);

}
