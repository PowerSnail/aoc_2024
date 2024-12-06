use aoc_2024;
use regex::Regex;

fn main() {
    let part = aoc_2024::get_part();
    let input = aoc_2024::get_stdin();

    if part == 1 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0;
        for m in re.captures_iter(&input) {
            let n1 = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let n2 = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
            sum += n1 * n2;
        }
        println!("{}", sum);
    } else {
        let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
        let mut enabled = true;
        let mut sum = 0;
        for m in re.captures_iter(&input) {
            if m.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if m.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            } else if enabled {
                let n1 = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let n2 = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
                sum += n1 * n2;
            }
        }
        println!("{}", sum);
    }
}
