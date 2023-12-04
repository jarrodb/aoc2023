
pub fn part1(_input: &Vec<String>) -> i32 {
    return 0;
}

pub fn part2(_input: &Vec<String>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day4::part1;
    use crate::day4::part2;

    fn test_data() -> Vec<String> {
        let data = vec![
            String::from(""),
        ];
        return data;
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
