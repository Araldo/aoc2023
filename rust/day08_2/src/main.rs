use std::collections::HashMap;
use num::integer::gcd;
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

    let mut nodes: Vec<&str> = vec![];
    for (k, _) in m.iter() {
        if k.chars().collect::<Vec<char>>()[2] == 'A' {
            nodes.push(k);
        }
    }

    let mut freq: [i64;6]= [0; 6];

    loop {
        if i >= instuctions.len() {
            i=0;
        }
        let inst = instuctions[i];
        i=i+1;
        j=j+1;
        // println!("{:?}", nodes);

        for idx in 0..nodes.len() {
            let pos = nodes[idx];
            let next = match inst {
                'R' => m.get(&pos).unwrap().1,
                'L' => m.get(&pos).unwrap().0,
                _ => "not reachable"
            };
            nodes[idx] = next;
        }

        if nodes.iter().all(|x| x.chars().collect::<Vec<char>>()[2] == 'Z') {
            break;
        }

        for idx in 0..nodes.len() {
            if nodes[idx].chars().collect::<Vec<char>>()[2] == 'Z' && freq[idx] == 0{
                freq[idx] = j;
            }
        }
        if freq.iter().all(|x| *x != 0) {
            break;
        }
   }
   let mut res = freq[0];
   for idx in 1..nodes.len() {
        res = res*freq[idx]/gcd(res, freq[idx]);
   }
   print!("{}", res);
}
