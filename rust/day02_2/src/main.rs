use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let games: Vec<&str> = input.split("\n").collect();
    let games: Vec<&str> = games.iter().map(
        |x| x.split(": ").last().unwrap()
    ).collect();
    let rounds: Vec<Vec<_>> = games.iter().map(
        |x| x.split(";").collect()
    ).collect();
    
    let blue = Regex::new(r"(\d+) blue").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let red = Regex::new(r"(\d+) red").unwrap();

    // get the number before the color and convert to int; the color might not be present, in which case it's 0
    let rounds: Vec<Vec<_>> = rounds.iter().map(
        |x| x.iter().map(
            |y| {
                let b = blue.captures(y).map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap()).unwrap_or(0);
                let g = green.captures(y).map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap()).unwrap_or(0);
                let r = red.captures(y).map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap()).unwrap_or(0);
                (b, g, r)
            }   
        ).collect()
    ).collect();

    let result: Vec<_> = rounds.iter().map(
        |x|
            x.iter().map(|y| y.0).max().unwrap() *
            x.iter().map(|y| y.1).max().unwrap() *
            x.iter().map(|y| y.2).max().unwrap()
        ).collect();

    println!("{:?}", result.iter().sum::<i32>());
}
