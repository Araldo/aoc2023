
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.split(" ").collect()).collect::<Vec<Vec<&str>>>();
    
    let patterns = lines.iter().map(|x| x[0].trim_matches('.').chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    // let patterns = patterns.iter().map(|x| [x.clone(), x.clone()].concat()).collect::<Vec<Vec<char>>>();
    let groups = lines.iter().map(
        |x| x[1].split(",").collect::<Vec<_>>().iter().map(
            |x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>();

    let question_marks = patterns.iter().map(
        |x| x.iter().map(|y| if y == &'?' {1} else {0}).sum::<i32>()
    ).collect::<Vec<i32>>();
    let hashes = patterns.iter().map(
        |x| x.iter().map(|y| if y == &'#' {1} else {0}).sum::<i32>()
    ).collect::<Vec<i32>>();

    let mut result = 0;
    for idx in 0..groups.len() {
        let x = vec![
            vec!['0'; (question_marks[idx] - (groups[idx].iter().sum::<i32>() - hashes[idx])).try_into().unwrap()],
            vec!['1'; (groups[idx].iter().sum::<i32>() - hashes[idx]).try_into().unwrap()]
        ].concat();

        println!("{:?}", x);
        for jdx in 0..2i32.pow(question_marks[idx].try_into().unwrap()) {
            let mut pattern = patterns[idx].clone();
            let mut pos = 0;
            for kdx in 0..question_marks[idx] {
                while pattern[pos] != '?' {
                    pos += 1;
                }
                if (jdx >> kdx) & 1 == 1 {
                    pattern[pos] = '#';
                } else {
                    pattern[pos] = '.';
                }
            }
            let mut group = vec![];
            let mut counter = 0;
            for (vdx, v) in pattern.iter().enumerate() {
                if v == &'#' {
                    counter += 1;
                } else {
                    if counter > 0 {
                        group.push(counter);
                        counter = 0;
                    }
                }
                if vdx == pattern.len() - 1 && counter > 0{
                    group.push(counter);
                }
            }
            if group == groups[idx] {
                result += 1;
            }
        }
    }
    println!("{:?}", result);
}
