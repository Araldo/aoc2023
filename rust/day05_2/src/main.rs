use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let seeds = lines.get(0).unwrap().split(":").collect::<Vec<&str>>().get(1).unwrap().trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let seeds= vec![(seeds[0], seeds[1]), (seeds[2], seeds[3]), (seeds[4], seeds[5]), (seeds[6], seeds[7]), (seeds[8], seeds[9]), (seeds[10], seeds[11]), (seeds[12], seeds[13]), (seeds[14], seeds[15]), (seeds[16], seeds[17]), (seeds[18], seeds[19])];

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
    let mut best_seed: i64 = 0;

    for (seed_base, range) in seeds.iter() {
        for seed in num::range_step(*seed_base, *seed_base + *range, 100000) {
            let mut current: i64 = seed.clone();
            for stage in 0..7 {
                for mapping in mappings[stage].iter() {
                    if current >= mapping[1] && current <= mapping[1] + mapping[2] - 1 {
                        current = current - mapping[1] + mapping[0];
                        break
                    }
                }
            }
            if current < min_location {
                min_location = current;
                best_seed = seed.clone();
            }
        }
    }


    // This is not accurate for all possible inputs; needs checks for containment in the given ranges, but it works without it for the input given.
    for seed in (best_seed-100000)..(best_seed) {
        let mut current: i64 = seed.clone();
        for stage in 0..7 {
            for mapping in mappings[stage].iter() {
                if current >= mapping[1] && current <= mapping[1] + mapping[2] - 1 {
                    current = current - mapping[1] + mapping[0];
                    println!("{} {} -> {}", stage, seed, current);
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
