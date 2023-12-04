use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut cards: Vec<i32> = vec![1; 201];    
    let re = Regex::new(r"(\d+)").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let split_lines = lines
        .iter()
        .map(|x|
            x.split(":").collect::<Vec<&str>>()
        );

    for (idx, split_line) in split_lines.enumerate() {
        let data = split_line[1].split("|").collect::<Vec<&str>>();
        let winners = re.captures_iter(data[0]).map(|x| x.get(1).unwrap().as_str()).collect::<HashSet<&str>>();
        let mine = re.captures_iter(data[1]).map(|x| x.get(1).unwrap().as_str()).collect::<HashSet<&str>>();
        let matches = winners.intersection(&mine).collect::<Vec<_>>().len();

        for i in 0..matches {
            if idx + i + 1 < cards.len() {
                cards[idx + i + 1] = cards[idx + i + 1] + cards[idx];
            }
        }
    }
    println!("{:?}", cards.iter().sum::<i32>());
}
