use std::char;


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
    let input = std::fs::read_to_string("input.txt").unwrap().split(',').map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut result=0;

    for element in input {
        result += box_id(element);
    }

    println!("{:?}", result)
}
