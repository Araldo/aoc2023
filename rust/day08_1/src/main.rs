use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let instuctions = lines[0].chars().collect::<Vec<char>>();

    let mappings = &lines[2..];
    let mut m = HashMap::new();

    for mapping in mappings {
        let origin = mapping.split(" = ").collect::<Vec<&str>>()[0];
        let target = mapping.split(" = ").collect::<Vec<&str>>()[1];
        let left: &str= &target[1..4];
        let right: &str = &target[6..9];
        m.insert(origin, (left, right));
    }

    let mut i=0;
    let mut j=0;
    let mut pos: &str = "AAA";
    loop {
        if i >= instuctions.len() {
            i=0;
        }
        let inst = instuctions[i];
        i=i+1;
        j=j+1;
        
        pos = match inst {
            'R' => m.get(&pos).unwrap().1,
            'L' => m.get(&pos).unwrap().0,
            _ => "not reachable"
        };
        if pos == "ZZZ" {
            break;
        }
        // }
    }
    println!("{}", j);
}
