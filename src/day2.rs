use aoc_2024;

fn is_safe<'a>(mut report: impl Iterator<Item = &'a i64>) -> bool {
    let mut prev_diff = 0;
    let mut prev_number = report.next().unwrap();
    for i in report {
        let diff = i - prev_number;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if prev_diff * diff < 0 {
            return false;
        }
        prev_diff = diff;
        prev_number = i;
    }
    true
}

fn is_safe_with_tolerance(report: &[i64]) -> bool {
    for i in 0..report.len() {
        if is_safe(report[..i].iter().chain(report[i + 1..].iter())) {
            return true;
        }
    }
    false
}

fn main() {
    let part = aoc_2024::get_part();
    let input = aoc_2024::get_stdin();

    let reports = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|segment| segment.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if part == 1 {
        let sum = reports
            .iter()
            .map(|v| is_safe(v.into_iter()))
            .fold(0, |a, v| if v { a + 1 } else { a });
        println!("{}", sum);
    } else {
        let sum = reports
            .iter()
            .map(|v| is_safe_with_tolerance(v.as_slice()))
            .fold(0, |a, v| if v { a + 1 } else { a });
        println!("{}", sum);
    }
}
