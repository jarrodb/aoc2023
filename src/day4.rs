#[derive(Clone, Copy)]
enum ParseMode {
    Card,
    Winning,
    Mine,
}

pub fn part1(input: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in input.iter() {
        let mut mode = ParseMode::Card;
        let mut buffer = String::new();
        let mut score: i32 = 0;
        let mut winning_cards: Vec<i32> = Vec::new();

        for c in line.chars() {
            if c.is_digit(10) {
                buffer.push(c);
                continue;
            }
            match (c, mode) {
                (':', ParseMode::Card) => {
                    mode = ParseMode::Winning;
                    //let card_id: i32 = buffer.parse().unwrap();
                    //println!("Card: {}", card_id);
                    buffer.clear();
                },
                ('|', ParseMode::Winning) => {
                    mode = ParseMode::Mine;
                    buffer.clear();
                },
                (_, ParseMode::Winning) => {
                    if !buffer.is_empty() {
                        let card: i32 = buffer.parse().unwrap();
                        winning_cards.push(card.clone());
                        buffer.clear();
                    }
                },
                (_, ParseMode::Mine) => {
                    if !buffer.is_empty() {
                        let card: i32 = buffer.parse().unwrap();
                        if winning_cards.contains(&card) {
                            let s = if score == 0 { 1 } else { score };
                            score += s;
                        }
                        buffer.clear();
                    }
                }
                _ => {},
            }
        }

        if !buffer.is_empty() {
            // don't repeat this
            let card: i32 = buffer.parse().unwrap(); 
            if winning_cards.contains(&card) {
                let s = if score == 0 { 1 } else { score };
                score += s;
            }
        }

        sum += score;
    }
    return sum;
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
        assert_eq!(part2(&test_data()), 0);
    }
}
