use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    x: i32,
    y: i32,
    text: String,
}

impl Entry {
    fn new() -> Entry {
        return Entry {x: 0, y: 0, text: String::new()};
    }

    fn surrounding_coordinates(&self) -> Vec<String> {
        let mut coordinates = Vec::<String>::new();
        for (i, _) in self.text.chars().enumerate() {
            // yes, super ghetto
            let new_x: i32 = self.x + (i as i32);
            coordinates.push(format!("{}_{}", new_x - 1, self.y)); // left
            coordinates.push(format!("{}_{}", new_x + 1, self.y)); // right
            coordinates.push(format!("{}_{}", new_x, self.y - 1)); // up
            coordinates.push(format!("{}_{}", new_x, self.y + 1)); // down
            coordinates.push(format!("{}_{}", new_x - 1, self.y - 1)); // upleft
            coordinates.push(format!("{}_{}", new_x + 1, self.y - 1)); // upright
            coordinates.push(format!("{}_{}", new_x - 1, self.y + 1)); // downleft
            coordinates.push(format!("{}_{}", new_x + 1, self.y + 1)); // downright
        }
        return coordinates;
    }
}

pub fn part1(input: &Vec<String>) -> i32 {
    let (entries, symbols) = parse_schematic(input);
    let mut sum = 0;

    for e in entries.iter() {
        // check all surrounding coordinates for a symbols match
        for c in e.surrounding_coordinates().iter() {
            if symbols.contains_key(c) {
                sum += e.text.parse::<i32>().unwrap();
                break; // go to next entry if matched
            }
        }
    }
    return sum;
}

pub fn part2(input: &Vec<String>) -> i32 {
    let (entries, symbols) = parse_schematic(input);
    let mut gears = HashMap::<String, Vec<i32>>::new();
    let mut sum = 0;

    for e in entries.iter() {
        // check all surrounding coordinates for a symbols match
        for c in e.surrounding_coordinates().iter() {
            match symbols.get(c) {
                Some(s) if s.to_string() == "*" => {
                    // append the entry part number at c
                    let value = e.text.parse::<i32>().unwrap();
                    gears.entry(c.to_string()).or_insert(Vec::<i32>::new()).push(value);
                    break; // go to next entry if matched
                },
                Some(_) => {},
                None => {},
            }
        }
    }

    // add the products of gears touching two "part numbers"
    for (_, v) in gears.iter() {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }
    return sum;
}

fn parse_schematic(input: &Vec<String>) -> (Vec<Entry>, HashMap<String, String>) {
    let mut entries = Vec::<Entry>::new(); // parts
    let mut symbols = HashMap::new(); // symbols
    for (y, line) in input.iter().enumerate() {
        let mut buffer = String::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if buffer.is_empty() {
                    // save the start x,y of the number
                    let mut entry = Entry::new();
                    entry.x = x as i32;
                    entry.y = y as i32;
                    entries.push(entry);
                }
                buffer.push(c); // build the number
                continue;
            } else if !buffer.is_empty() {
                // save the number
                let entry = entries.last_mut().unwrap();
                entry.text = buffer.clone();
                buffer.clear();
            }

            // ignore . and new lines
            if c != '.' && c != '\n' {
                // save the symbol x,y
                let key = format!("{}_{}", x, y);
                symbols.insert(key.clone(), c.to_string());
            }
        }
    }
    return (entries, symbols);
}

#[cfg(test)]
mod tests {
    use crate::day3::part1;
    use crate::day3::part2;

    fn test_data() -> Vec<String> {
        let data = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];
        return data;
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_data()), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_data()), 467835);
    }
}