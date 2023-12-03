use regex::Regex;
use core::cmp::max;
use core::cmp::min;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.split("\n").collect();
    let mut sum: i32 = 0;
    
    let has_symbol = Regex::new(r"[^\d\.]").unwrap();

    for (idx, line) in lines.iter().enumerate() {
        let captures = Regex::new(r"(\d+)").unwrap();
        
        for cap in captures.captures_iter(line) {
            let start = cap.get(1).unwrap().start();
            let end = cap.get(1).unwrap().end();
            
            let previous_line = &lines[max(idx as i32 - 1, 0) as usize];
            let next_line = &lines[min(idx+1, lines.len()-1)];

            for surrounding_line in vec![previous_line, line, next_line] {
                let surrounding_fields = &surrounding_line[(max((start as i32)-1, 0) as usize)..min(end+1, surrounding_line.len())];
                if has_symbol.captures(surrounding_fields).is_some() {
                    sum += cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    break;
                }
            }

        }
    }
    println!("{}", sum);
}
