
#[allow(dead_code)]
pub enum Data {
    Test,
    Real,
}

fn test_data() -> Vec<(i32, i32)> {
    // (time, distance)
    return vec![
        (7, 9),
        (15, 40),
        (30, 200),
    ];
}

fn real_data() -> Vec<(i32, i32)> {
    // (time, distance)
    return vec![
        (48, 255),
        (87, 1288),
        (69, 1117),
        (81, 1623),
    ];
}

pub fn part1(input: Data) -> i32 {
    // holding down button charges boat
    // release button moves boat 
    // moves fast if button held down longer
    // 7 ms -> 9 mm

    let data = match input {
        Data::Test => test_data(),
        _ => real_data(),
    };

    let mut times: Vec<i32> = Vec::new();

    for (time, distance) in data {
        let mut records: i32 = 0;
        for hold_t in 1..time {
            if hold_t * (time - hold_t) > distance {
                records += 1;
            }
        }
        times.push(records);
    }

    return times.iter().product();
}

pub fn part2(_input: Data) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day6::part1;
    use crate::day6::part2;
    use crate::day6::Data;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Data::Test), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Data::Test), 0);
    }
}