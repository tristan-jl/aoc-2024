const INPUT: &str = include_str!("../../inputs/day3.txt");

fn part1(mut input: &str) -> u32 {
    let mut score = 0;

    while input.len() > 8 {
        if let Some((_, r)) = input.split_once("mul(") {
            if let Some((check, rest)) = r.split_once(")") {
                if let Some((left, right)) = check.split_once(",") {
                    if let (Ok(leftd), Ok(rightd)) = (left.parse::<u32>(), right.parse::<u32>()) {
                        score += leftd * rightd;
                        input = rest;
                        continue;
                    }
                }
            }
        }
        input = input.split_at(1).1;
    }

    score
}

fn part2(mut input: &str) -> u32 {
    let mut score = 0;
    let mut is_do = true;

    while input.len() > 8 {
        if input.starts_with("mul(") {
            if let Some((_, r)) = input.split_once("mul(") {
                if let Some((check, rest)) = r.split_once(")") {
                    if let Some((left, right)) = check.split_once(",") {
                        if let (Ok(leftd), Ok(rightd)) = (left.parse::<u32>(), right.parse::<u32>())
                        {
                            if is_do {
                                score += leftd * rightd;
                            }
                            input = rest;
                            continue;
                        }
                    }
                }
            }
        } else if input.starts_with("do()") {
            input = input.split_once("do()").expect("just checked").1;
            is_do = true;
            continue;
        } else if input.starts_with("don't()") {
            input = input.split_once("don't()").expect("just checked").1;
            is_do = false;
            continue;
        }
        input = input.split_at(1).1;
    }

    score
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
