fn main() {
    // read input text file into a string: input.txt
    let input = std::fs::read_to_string("../day01_1/input.txt").unwrap();
    
    println!("{:?}", input);
    
    // replace the word "six" with the digit "6" in the input string
    let input = input
    .replace("oneight", "18")
    .replace("twone", "21")
    .replace("threeight", "38")
    .replace("fiveight", "58")
    .replace("sevenine", "79")
    .replace("eightwo", "82")
    .replace("eighthree", "83")
    .replace("nineight", "98")
    .replace("one", "1")
    .replace("two", "2")
    .replace("three", "3")
    .replace("four", "4")
    .replace("five", "5")
    .replace("six", "6")
    .replace("seven", "7")
    .replace("eight", "8")
    .replace("nine", "9");
    
    println!("{:?}", input);

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