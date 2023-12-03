use std::vec;
use std::collections::HashMap;
use regex::Regex;
use core::cmp::max;
use core::cmp::min;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = input.split("\n").collect();

    let mut star_positions: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (idx, line) in lines.iter().enumerate() {
        let captures = Regex::new(r"(\d+)").unwrap();
        
        for cap in captures.captures_iter(line) {
            let start = cap.get(1).unwrap().start();
            let end = cap.get(1).unwrap().end();
            
            let previous_line = &lines[max(idx as i32 - 1, 0) as usize];
            let next_line = &lines[min(idx+1, lines.len()-1)];

            for (x, surrounding_line) in vec![previous_line, line, next_line].iter().enumerate() {
                let surrounding_field_start = max((start as i32)-1, 0) as usize;
                let surrounding_fields = &surrounding_line[surrounding_field_start..min(end+1, surrounding_line.len())];
                
                let star_pos = surrounding_fields.find("*");
                if star_pos.is_some() {
                    let position = (idx + x - 1, star_pos.unwrap() + surrounding_field_start);
                    star_positions.entry(position).or_insert(vec![]);
                    star_positions
                        .entry(position)
                        .and_modify(
                            |e|
                            e.push(cap.get(1).unwrap().as_str().parse::<i32>().unwrap())
                        );
                }
            }
        }
    }

    println!("{:?}", star_positions.iter().map(|(_, v)| if v.len() == 2 { v[0] * v[1] } else { 0 }).sum::<i32>());
}
