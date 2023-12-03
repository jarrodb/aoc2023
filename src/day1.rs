use std::collections::HashMap;

pub fn part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in input.iter() {
        sum += calibration_value(line);
    }

    return sum;
}

pub fn part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in input.iter() {
        sum += calibration_value_v2(line);
    }

    return sum;
}

fn calibration_value_v2(input: &str) -> u32 {
    let mut numbers = Vec::new();
    let number_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut i = 0;
    while i < input.len() {
        let c = input.chars().nth(i).unwrap();
        if c.is_digit(10) {
            numbers.push(c);
            i += 1;
        } else {
            let mut add = 1;
            for (key, value) in number_map.iter() {
                if (input.len() - i) < key.len() {
                    continue;
                } else if input[i..].starts_with(key) {
                    numbers.push(*value);
                    add = key.len() - 1;
                    continue;
                }
            }
            i += add;
        }
    }

    return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<u32>()
        .unwrap();
}

fn calibration_value(input: &str) -> u32 {
    let mut numbers = Vec::new();

    for c in input.chars() {
        if c.is_digit(10) {
            numbers.push(c);
        }
    }

    return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<u32>()
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];
        assert_eq!(super::part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        assert_eq!(super::part2(&input), 281);
    }
}