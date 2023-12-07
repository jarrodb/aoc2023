#[derive(Debug)]
struct Map {
    src: i128,
    dst: i128,
    range: i128,
}

impl Map {
    fn destination(&self, src: i128) -> i128 {
        if self.src <= src && src < (self.src + self.range) {
            return self.dst + (src - self.src);
        }
        return -1; // not found
    }
}

fn parse_seeds(input: &str) -> Vec<i128> {
    return input
        .trim()
        .split(" ")
        .filter_map(|e| e.parse::<i128>().ok())
        .collect();
}

fn parse_sections(input: &Vec<String>) -> Vec<Vec<Map>> {
    let mut sections: Vec<Vec<Map>> = Vec::new();

    let mut maps = Vec::<Map>::new();

    for line in input.iter().skip(2) {
        if line.trim().is_empty() {
            // skip empty lines
            continue;
        }

        if !line.chars().nth(0).unwrap().is_digit(10) {
            // moving to the next map, append current to sections
            if maps.len() > 0 {
                sections.push(maps);
            }
            maps = Vec::<Map>::new();

            continue;
        }

        let entry: Vec<i128> = line
            .trim()
            .split(" ")
            .filter_map(|e| e.parse::<i128>().ok())
            .collect();
        let m = Map{dst: entry[0], src: entry[1], range: entry[2]};
        maps.push(m);
    }

    // push the last map
    if maps.len() > 0 {
        sections.push(maps);
    }

    return sections;
}

fn location_number(seed: i128, sections: &Vec<Vec<Map>>) -> i128 {
    let mut src = seed;
    for section in sections.iter() { // translate for each section map
        for map in section.iter() { // check if valid destination
            let dst = map.destination(src);
            if dst > -1 {
                src = dst;
                break;
            }
        }
    }
    return src;
}

pub fn part1(input: &Vec<String>) -> i128 {
    let seeds = parse_seeds(&input[0]);
    let sections = parse_sections(&input);
    let mut lowest = 0;

    for seed in seeds.iter() { // iterate through all seeds
        let src = location_number(*seed, &sections);

        lowest = if lowest == 0 { src } else { std::cmp::min(lowest, src) };
    }

    return lowest;
}

pub fn part2(input: &Vec<String>) -> i128 {
    let seeds = parse_seeds(&input[0]);
    let sections = parse_sections(&input);
    let mut lowest = 0;

    for (i, data) in seeds.iter().step_by(2).enumerate() { // iterate through all seeds
        for seed in *data..*data+seeds[i*2+1]-1 {
            let src = location_number(seed, &sections);

            lowest = if lowest == 0 { src } else { std::cmp::min(lowest, src) };
        }
    }

    return lowest;
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
        assert_eq!(part1(&test_data()), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_data()), 46);
    }

    #[test]
    fn test_map_destination() {
        let mut m = crate::day5::Map{dst: 43048329, src: 2909431, range: 10};
        assert_eq!(m.destination(2909431), 43048329);
        assert_eq!(m.destination(2909433), 43048331);
        assert_eq!(m.destination(0), -1);

        m = crate::day5::Map{dst: 0, src: 2909431, range: 10};
        assert_eq!(m.destination(2909432), 1);

        m = crate::day5::Map{dst: 94302, src: 0, range: 10};
        assert_eq!(m.destination(7), 94309);
    }
}
