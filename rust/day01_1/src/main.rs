fn main() {
    // read input text file into a string: input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // filter out all non-digits and newlines
    let filtered_string = input.chars()
        .filter(|x| x.is_digit(10) || x == &'\n')
        .into_iter()
        .collect::<String>();

    // split string into vector of strings, each string is a line of digits
    let digits = filtered_string.split("\n").filter(|x| x.len() > 0).collect::<Vec<&str>>();

    // convert each string to a vector of chars, take the first and last one, convert each char to a digit, then sum the digits
    let first_plus_last: u32 = digits
        .iter()
        .map(
            |x|
            x.chars().nth(0).unwrap().to_digit(10).unwrap() * 10 +
            x.chars().last().unwrap().to_digit(10).unwrap()
        )
        .sum();
    println!("{:?}", first_plus_last);
}