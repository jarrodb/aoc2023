
pub fn part1(_input: &Vec<String>) -> i32 {
    return 0;
}

pub fn part2(_input: &Vec<String>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day5::part1;
    use crate::day5::part2;

    fn test_data() -> Vec<String> {
        let data = vec![
            String::from("seeds: 79 14 55 13"),
            String::from(""),
            String::from("seed-to-soil map:"),
            String::from("50 98 2"),
            String::from("52 50 48"),
            String::from(""),
            String::from("soil-to-fertilizer map:"),
            String::from("0 15 37"),
            String::from("37 52 2"),
            String::from("39 0 15"),
            String::from(""),
            String::from("fertilizer-to-water map:"),
            String::from("49 53 8"),
            String::from("0 11 42"),
            String::from("42 0 7"),
            String::from("57 7 4"),
            String::from(""),
            String::from("water-to-light map:"),
            String::from("88 18 7"),
            String::from("18 25 70"),
            String::from(""),
            String::from("light-to-temperature map:"),
            String::from("45 77 23"),
            String::from("81 45 19"),
            String::from("68 64 13"),
            String::from(""),
            String::from("temperature-to-humidity map:"),
            String::from("0 69 1"),
            String::from("1 0 69"),
            String::from(""),
            String::from("humidity-to-location map:"),
            String::from("60 56 37"),
            String::from("56 93 4"),
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
