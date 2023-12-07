use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .replace("A", "e")
        .replace("K", "d")
        .replace("Q", "c")
        .replace("J", "1")
        .replace("T", "a");


    let mut hands: Vec<(String, i32)> = vec![];

    for line in input.lines() {
        let data = line.split(" ").collect::<Vec<&str>>();
        let value = data[0];
        let bet = data[1].parse::<i32>().unwrap();

        let mut hand_value = "0";

        for replacement in "23456789acde".chars() {
            let jokers_subs_value = value.replace("1", replacement.to_string().as_str());

            let mut sorted_hand = jokers_subs_value.chars().collect::<Vec<char>>();
            sorted_hand.sort();
            let counts = sorted_hand.into_iter().counts();
            let max_dup = counts.values().max().unwrap();

            let replacement_hand_value = match (counts.len(), max_dup) {
                    (5, 1) => "1",
                    (4, 2) => "2",
                    (3, 2) => "3",
                    (3, 3) => "4",
                    (2, 3) => "5",
                    (2, 4) => "6",
                    (1, 5) => "7",
                    _ => "0"
            };
            if replacement_hand_value > hand_value {
                hand_value = replacement_hand_value;
            }
        }

        let value: String = format!("{}{}",hand_value, value).as_str().to_owned();
        
        hands.push((value.clone(), bet));
    }
    hands.sort();

    let result = hands.iter().enumerate().map(|(idx, (_, y))| ((idx as i32)+1) * y).sum::<i32>();
    println!("{:?}", result);
}
