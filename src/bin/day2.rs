const INPUT: &str = include_str!("../../inputs/day2.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|i| {
            i.split_whitespace()
                .into_iter()
                .map(|i| i.parse().expect("couldn't parse number"))
                .collect()
        })
        .collect()
}

fn check_report(report: &[u32]) -> bool {
    let is_increase = report[1] > report[0];
    let mut it = report.into_iter();
    let mut last_num = it.next().expect("should have at least 1 number in report");

    for i in it {
        if i == last_num {
            return false;
        } else if is_increase && i < last_num || !is_increase && i > last_num {
            return false;
        } else if last_num.abs_diff(*i) > 3 {
            return false;
        }
        last_num = i
    }
    true
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .into_iter()
        .map(|r| check_report(&r) as u32)
        .sum()
}

fn check_with_rm(r: &[u32]) -> bool {
    if check_report(&r) {
        return true;
    }

    for i in 0..r.len() {
        let mut nr = r.to_vec();
        nr.remove(i);
        if check_report(&nr) {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .into_iter()
        .map(|mut r| check_with_rm(&mut r) as u32)
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IN: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_IN), 2);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_IN), 4);
    }
}
