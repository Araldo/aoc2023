use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let seeds = lines.get(0).unwrap().split(":").collect::<Vec<&str>>().get(1).unwrap().trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut mappings: Vec<Vec<Vec<i64>>>= vec![];
    for _ in 0..7 {
        mappings.push(vec![]);
    }

    let mut counter: usize = 0;
    for line in lines.iter().skip(1) {
        if Regex::new(r"\d").unwrap().is_match(line) {

            let data = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            mappings[counter-1].push(data);
        }
        else if Regex::new(r":").unwrap().is_match(line) {
            counter += 1;
        }
    }

    let mut min_location: i64 = i64::MAX;

    for seed in seeds.iter() {
        let mut current = seed.clone();
        for stage in 0..7 {
            for mapping in mappings[stage].iter() {
                if current >= mapping[1] && current <= mapping[1] + mapping[2] - 1 {
                    current = current - mapping[1] + mapping[0];
                    println!("{} {} {}", seed, stage, current);
                    break
                }
            }
        }
        if current < min_location {
            min_location = current;
        }
    }

    println!("{:?}", min_location);
}
