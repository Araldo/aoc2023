fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result: i32 = 0;
    for line in lines {
        let mut rows: Vec<Vec<i32>> = vec![];
        let values = line.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        rows.push(values);
        loop {
            let v = rows.last().unwrap();
            let mut row: Vec<i32> = vec![];
            for idx in 0..v.len()-1 {
                row.push(v[idx+1] - v[idx]);
            }
            rows.push(row.clone());
            if row.iter().all(|x| *x == 0) {
                break;
            }
        }
        let mut dif: i32 = 0; 
        for idx in (0..rows.len()-1).rev() {
            dif = rows[idx].last().unwrap() + dif;
            rows[idx].push(dif);
        }
        result += dif;
    }
    println!("result: {}", result);
}
