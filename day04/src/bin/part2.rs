use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");
    print!("Result: {}\n", process(input));
}

fn process(input: &str) -> u32 {
    let counts: Vec<(usize, u32)> = input
        .lines()
        .enumerate()
        .map(|(game_id, line)| {
            let without_prefix = line.split_once(":").unwrap().1;
            let mut parts = without_prefix.split(" | ").map(|part| {
                HashSet::<u32>::from_iter(
                    part.split(" ")
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<u32>().unwrap()),
                )
            });

            let winning = parts.next().unwrap();
            let numbers = parts.next().unwrap();

            let count = numbers.intersection(&winning).count() as u32;
            (game_id, count)
        })
        .collect();

    let mut total_winning_cards = counts.len() as u32;

    let mut games = counts.clone();

    while games.len() > 0 {
        let (game_id, winning_count) = games.pop().unwrap();
        for i in 0..winning_count {
            let ticket_nr = game_id + 1 + i as usize;
            games.push(counts[ticket_nr]);
            total_winning_cards += 1;
        }
    }

    total_winning_cards as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(process(input), 30);
    }
}
