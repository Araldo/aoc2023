use std::collections::HashMap;

fn box_id(label: Vec<char>) -> u32 {
    let mut result: u32 = 0;
    for c in label {
        result += c as u8 as u32;
        result *= 17;
        result %= 256;
    }
    result.into()
}

#[test]
fn test_box_id() {
    assert_eq!(box_id(vec!['r', 'n']), 0);
    assert_eq!(box_id(vec!['q', 'p']), 1);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap().split(',')
        .map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut boxes: HashMap::<u32, Vec::<(Vec<char>, u32)>> = HashMap::new();

    for i in 0..=255 {
        boxes.insert(i, vec![]);
    }

    let mut label;
    for element in input {
        label = vec![];
        'element: for c in element.clone() {
            if c == '=' {
                let lens = element.last().unwrap().clone().to_digit(10).unwrap();
                for b in boxes.get_mut(&box_id(label.clone())).unwrap() {
                    if b.0 == label {
                        b.1 = lens;
                        break 'element;
                    }
                }
                boxes.get_mut(&box_id(label.clone())).unwrap().push((label.clone(), lens));
            } else if c == '-' {
                let b = boxes.get_mut(&box_id(label.clone())).unwrap();
                for i in 0..b.len() {
                    if b[i].0 == label {
                        b.remove(i);
                        break 'element;
                    }
                }
            }
            label.push(c);
        }
    }
    let mut result = 0;
    for i in 0..=255 {
        for (idx, b) in boxes.get(&i).unwrap().iter().enumerate() {
            result += b.1 * ((i+1) as u32) * ((idx+1) as u32);
        }
    }
    println!("{}", result)
}