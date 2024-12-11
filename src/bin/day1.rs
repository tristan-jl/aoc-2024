use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|i| {
            let mut nums = i.split_whitespace();
            let first: u32 = nums
                .next()
                .expect("didn't get first number")
                .parse()
                .expect("couldn't parse first string");
            let second: u32 = nums
                .next()
                .expect("didn't get second number")
                .parse()
                .expect("couldn't parse second string");

            (first, second)
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let (mut l, mut r) = parse_input(input);
    l.sort_unstable();
    r.sort_unstable();

    l.into_iter()
        .zip(r.into_iter())
        .map(|(li, ri)| li.abs_diff(ri))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (l, r) = parse_input(input);

    let mut counts: HashMap<u32, u32> = HashMap::with_capacity(l.len() as usize);

    fn count_occurences(v: &Vec<u32>, t: u32) -> u32 {
        let mut count = 0;
        for &n in v {
            if n == t {
                count += 1;
            }
        }

        count
    }

    l.into_iter()
        .map(|i| {
            if let Some(count) = counts.get(&i) {
                i * count
            } else {
                let count = count_occurences(&r, i);
                counts.insert(i, count);
                i * count
            }
        })
        .sum::<u32>()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IN: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_IN), 11);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_IN), 31);
    }
}
