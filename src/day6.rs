
#[allow(dead_code)]
pub enum Data {
    Test,
    Real,
}

fn test_data() -> Vec<(i64, i64)> {
    // (time, distance)
    return vec![
        (7, 9),
        (15, 40),
        (30, 200),
    ];
}

fn real_data() -> Vec<(i64, i64)> {
    // (time, distance)
    return vec![
        (48, 255),
        (87, 1288),
        (69, 1117),
        (81, 1623),
    ];
}

fn calculate_records(time: i64, distance: i64) -> i64 {
    let mut records: i64 = 0;
    for hold_t in 1..time {
        if hold_t * (time - hold_t) > distance {
            records += 1;
        }
    }
    return records
}

pub fn part1(input: Data) -> i64 {
    let data = match input {
        Data::Test => test_data(),
        _ => real_data(),
    };

    let mut times: Vec<i64> = Vec::new();

    for (time, distance) in data {
        let records = calculate_records(time, distance);
        times.push(records);
    }

    return times.iter().product();
}

pub fn part2(input: Data) -> i64 {
    let data = match input {
        Data::Test => test_data(),
        _ => real_data(),
    };

    let time = data
        .iter()
        .map(|x| x.0.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let distance = data
        .iter()
        .map(|x| x.1.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    return calculate_records(time, distance);
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
        assert_eq!(part2(Data::Test), 71503);
    }
}