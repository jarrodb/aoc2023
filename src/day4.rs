#[derive(Clone, Copy)]
enum ParseMode {
    Card,
    Winning,
    Mine,
}

struct Card {
    id: i32,
    winning: Vec<i32>,
    mine: Vec<i32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let mut scratch = Card {
            id: 0,
            winning: Vec::<i32>::new(),
            mine: Vec::<i32>::new()
        };

        let mut mode = ParseMode::Card;
        let mut buffer = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                buffer.push(c);
                continue;
            }

            match (c, mode) {
                (':', ParseMode::Card) => {
                    mode = ParseMode::Winning; // transition mode
                    scratch.id = buffer.parse::<i32>().unwrap();
                    buffer.clear();
                },
                ('|', ParseMode::Winning) => {
                    mode = ParseMode::Mine; // transition mode
                    buffer.clear();
                },
                (_, ParseMode::Winning) => {
                    if let Ok(num) = buffer.parse::<i32>() {
                        scratch.winning.push(num.clone());
                        buffer.clear();
                    }
                },
                (_, ParseMode::Mine) => {
                    if let Ok(num) = buffer.parse::<i32>() {
                        scratch.mine.push(num.clone());
                        buffer.clear();
                    }
                }
                _ => {},
            }
        }

        // process the last number
        if let Ok(num) = buffer.parse::<i32>() {
            scratch.mine.push(num.clone());
            buffer.clear();
        }

        return scratch;
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for card in self.winning.iter() {
            if self.mine.contains(card) {
                score += if score == 0 { 1 } else { score };
            }
        }
        return score;
    }

    fn total_matches(&self) -> i32 {
        let mut matches = 0;
        for card in self.winning.iter() {
            if self.mine.contains(card) {
                matches += 1;
            }
        }
        return matches;
    }
}

pub fn part1(input: &Vec<String>) -> i32 {
    return input
        .iter()
        .map(|line| Card::new(line).score())
        .sum();
}

pub fn part2(input: &Vec<String>) -> i32 {
    let cards: Vec<i32> = input
        .iter()
        .map(|line| Card::new(line).total_matches())
        .collect();

    // i got help online for the rest of this function
    let mut card_count: Vec<i32> = vec![1; cards.len()];

    for i in 0..cards.len() {
        let matches = cards[i] as usize;
        for j in (i + 1)..std::cmp::min(i + 1 + matches, cards.len()) {
            card_count[j] += card_count[i];
        }
    }

    return card_count.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::day4::part1;
    use crate::day4::part2;

    fn test_data() -> Vec<String> {
        let data = vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];
        return data;
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_data()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_data()), 30);
    }
}
