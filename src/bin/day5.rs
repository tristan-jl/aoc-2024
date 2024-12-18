use std::{cmp::Ordering, collections::HashSet};

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_raw, pn_raw) = input.split_once("\n\n").expect("must have 2 sections");

    let rules = rules_raw
        .lines()
        .map(|i| {
            let ns = i.split_once("|").expect("line doesn't contain |");
            (
                ns.0.parse::<u32>().expect("not a num"),
                ns.1.parse::<u32>().expect("not a num"),
            )
        })
        .collect();

    let pn = pn_raw
        .lines()
        .map(|i| {
            i.split(",")
                .map(|j| j.parse().expect("not a num"))
                .collect()
        })
        .collect();

    (rules, pn)
}

fn part1(input: &str) -> u32 {
    let (rules, pn) = parse_input(input);

    pn.iter()
        .flat_map(|i| {
            let mut new = i.clone();
            new.sort_by(|&a, &b| {
                if a == b {
                    Ordering::Equal
                } else if rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            if *i == new {
                Some(i[i.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, pn) = parse_input(input);

    pn.iter()
        .flat_map(|i| {
            let mut new = i.clone();
            new.sort_by(|&a, &b| {
                if a == b {
                    Ordering::Equal
                } else if rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            if *i == new {
                None
            } else {
                Some(new[new.len() / 2])
            }
        })
        .sum()
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
            part1(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            123
        );
    }
}
