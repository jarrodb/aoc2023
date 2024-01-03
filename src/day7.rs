pub fn part1(input: &Vec<String>) -> i128 {
    for line in input.iter() {
        let _hand = line.chars().collect::<String>();
    }
    return 0;
}

pub fn part2(_input: &Vec<String>) -> i128 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day7::part1;
    use crate::day7::part2;

    fn test_data() -> Vec<String> {
        return vec![
            String::from("32T3K 765"),
            String::from("T55J5 684"),
            String::from("KK677 28"),
            String::from("KTJJT 220"),
            String::from("QQQJA 483"),
        ];
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_data()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_data()), 0);
    }
}
