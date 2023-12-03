struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

impl Game {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    fn new() -> Game {
        return Game {id: 0, sets: Vec::new()}
    }

    fn add_set(&mut self, set: GameSet) {
        self.sets.push(set);
    }

    fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if set.red > Game::MAX_RED || set.green > Game::MAX_GREEN || set.blue > Game::MAX_BLUE {
                return false;
            }
        }
        return true;
    }

    fn minimum(&self) -> (i32, i32, i32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in self.sets.iter() {
            if set.red > red {
                red = set.red;
            }

            if set.green > green {
                green = set.green;
            }

            if set.blue > blue {
                blue = set.blue;
            }
        }
        return (red, green, blue);
    }
}

struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn part1(input: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input.iter() {
        let game = parse_game(line);
        if game.is_possible() {
            sum += game.id;
        }
    }
    return sum;
}

pub fn part2(input: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input.iter() {
        let game = parse_game(line);
        let (red, green, blue) = game.minimum();
        sum += red * green * blue;
    }
    return sum;
}

fn parse_game(input: &str) -> Game {
    let mut game = Game::new();
    let (mut red, mut green, mut blue): (i32, i32, i32) = (0, 0, 0);

    let mut buffer = String::new();
    let mut last_char = char::default();

    for c in input.chars() {
        if c.is_digit(10) {
            buffer.push(c);
            last_char = c;
            continue
        }

        match c {
            ':' => {
                // capture the game id from buffer and reset
                game.id = buffer.parse::<i32>().unwrap();
                buffer.clear();
            },
            'r' => { // red
                if last_char == ' ' {
                    red = buffer.parse::<i32>().unwrap();
                    buffer.clear();
                }
            },
            'g' => { // green
                if last_char == ' ' {
                    green = buffer.parse::<i32>().unwrap();
                    buffer.clear();
                }
            },
            'b' => { // blue
                if last_char == ' ' {
                    blue = buffer.parse::<i32>().unwrap();
                    buffer.clear();
                }
            },
            ';' => {
                // game set finished; save and reset
                game.add_set(GameSet {red: red, green: green, blue: blue});
                (red, green, blue) = (0, 0, 0);
            }
            _ => {},
        }

        last_char = c;
    }

    // we need to capture the last game too
    game.add_set(GameSet {red: red, green: green, blue: blue});

    return game
}

#[cfg(test)]
mod tests {
    use crate::day2::part1;
    use crate::day2::part2;

    fn test_data() -> Vec<String> {
        return vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_data()), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_data()), 2286);
    }
}