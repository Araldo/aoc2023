use std::vec;
use std::f64;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut times: Vec<&str> = vec![];
    let mut distances: Vec<&str>= vec![];

    for part in lines[0].split(" ").collect::<Vec<&str>>() {if part != "" {times.push(part)};}
    for part in lines[1].split(" ").collect::<Vec<&str>>() {if part != "" {distances.push(part)};}

    let times = &times[1..].iter().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    let distances = &distances[1..].iter().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>();

    let mut result= 1.0f64;

    for i in 0..times.len() {
        let x1 = 0.5f64 * times[i] - 0.5f64 * (times[i] * times[i] - 4f64 * distances[i]).sqrt();
        let x2 = 0.5f64 * times[i] + 0.5f64 * (times[i] * times[i] - 4f64 * distances[i]).sqrt();
        let x = (x2.ceil() - x1.floor()) - 1.0;
        print!("{} {} {}\n", x1, x2, x);
        result *= x;
    }
    print!("{}", result)
}
