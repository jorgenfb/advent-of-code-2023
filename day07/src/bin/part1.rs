fn main() {
    println!("Result: {}\n", process(include_str!("input.txt")));
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [u8; 5], // Should be sorted
    bet: u32,
}

impl Hand {
    fn from(input: &str) -> Self {
        let (cards_str, bet_str) = input.split_once(" ").unwrap();

        let cards: [u8; 5] = cards_str
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let mut sorted_cards = cards.clone();
        sorted_cards.sort_unstable();

        // Determine hand type
        let hand_type = if sorted_cards[0] == sorted_cards[4] {
            HandType::FiveOfAKind
        } else if sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4] {
            HandType::FourOfAKind
        } else if sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4] {
            // 3 + 2
            HandType::FullHouse
        } else if sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4] {
            // 2 + 3
            HandType::FullHouse
        } else if sorted_cards[0] == sorted_cards[2]
            || sorted_cards[1] == sorted_cards[3]
            || sorted_cards[2] == sorted_cards[4]
        {
            HandType::ThreeOfAKind
        } else if (sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[3])
            || (sorted_cards[0] == sorted_cards[1] && sorted_cards[3] == sorted_cards[4])
            || (sorted_cards[1] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4])
        {
            HandType::TwoPairs
        } else if sorted_cards[0] == sorted_cards[1]
            || sorted_cards[1] == sorted_cards[2]
            || sorted_cards[2] == sorted_cards[3]
            || sorted_cards[3] == sorted_cards[4]
        {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Self {
            cards,
            bet: bet_str.parse().unwrap(),
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        }

        for i in 0..5 {
            if self.cards[i] > other.cards[i] {
                return std::cmp::Ordering::Greater;
            } else if self.cards[i] < other.cards[i] {
                return std::cmp::Ordering::Less;
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

fn process(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from(line))
        .collect::<Vec<_>>();

    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bet)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(process(input), 6440);
    }
}
