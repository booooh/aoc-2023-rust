mod part1 {
    use std::collections::HashMap;

    use common::read_lines;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl From<&char> for Card {
        fn from(value: &char) -> Self {
            match value {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("This should never happen"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    struct Hand {
        cards: [Card; 5],
    }

    impl From<&str> for Hand {
        fn from(value: &str) -> Self {
            return Hand {
                cards: value
                    .chars()
                    .map(|c| (&c).into())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
            };
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAkind,
        FullHouse,
        FourOfAKind,
        FiveOfAkind,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let self_type = self.hand_type();
            let other_type = other.hand_type();
            let type_ord = self_type.partial_cmp(&other_type);
            match type_ord {
                Some(std::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
                _ => type_ord,
            }
        }
    }

    impl Hand {
        fn hand_type(&self) -> HandType {
            let mut counts = HashMap::<Card, u32>::new();
            for c in self.cards.iter() {
                *counts.entry(c.clone()).or_insert(0) += 1;
            }
            let mut sorted_counts = counts.values().collect::<Vec<_>>();
            sorted_counts.sort();
            sorted_counts.reverse();

            match (sorted_counts.len(), sorted_counts.first().unwrap()) {
                (1, _) => HandType::FiveOfAkind,
                (2, 4) => HandType::FourOfAKind,
                (2, 3) => HandType::FullHouse,
                (3, 3) => HandType::ThreeOfAkind,
                (3, 2) => HandType::TwoPair,
                (4, _) => HandType::Pair,
                _ => HandType::HighCard,
            }
        }
    }

    fn parse_line(l: String) -> (Hand, usize) {
        let mut parts = l.split(" ");
        let hand: Hand = parts.next().unwrap().into();
        let bid = parts.next().unwrap().parse().unwrap();

        (hand, bid)
    }

    pub(crate) fn part1() {
        let lines = read_lines("day07/input").unwrap();
        let mut bids = lines
            .map(|line| parse_line(line.unwrap()))
            .collect::<Vec<_>>();
        bids.sort();
        let winnings = bids
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum::<usize>();

        println!("{}", winnings);
    }
}

mod part2 {
    use std::collections::HashMap;

    use common::read_lines;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl From<&char> for Card {
        fn from(value: &char) -> Self {
            match value {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Joker,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("This should never happen"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    struct Hand {
        cards: [Card; 5],
    }

    impl From<&str> for Hand {
        fn from(value: &str) -> Self {
            return Hand {
                cards: value
                    .chars()
                    .map(|c| (&c).into())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
            };
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAkind,
        FullHouse,
        FourOfAKind,
        FiveOfAkind,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let self_type = self.hand_type();
            let other_type = other.hand_type();
            let type_ord = self_type.partial_cmp(&other_type);
            match type_ord {
                Some(std::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
                _ => type_ord,
            }
        }
    }

    impl Hand {
        fn hand_type(&self) -> HandType {
            let mut counts = HashMap::<Card, u32>::new();
            for c in self.cards.iter() {
                *counts.entry(c.clone()).or_insert(0) += 1;
            }

            // hanlde Jokers - count how many we have, and then remove them from the map
            let num_jokers = counts.remove(&Card::Joker).or(Some(0)).unwrap();

            let mut sorted_counts = counts.values().collect::<Vec<_>>();
            sorted_counts.sort();
            sorted_counts.reverse();

            match (
                sorted_counts.len(),
                *sorted_counts.first().or(Some(&&0u32)).unwrap() + num_jokers,
            ) {
                (1, _) | (0, _) => HandType::FiveOfAkind,
                (2, 4) => HandType::FourOfAKind,
                (2, 3) => HandType::FullHouse,
                (3, 3) => HandType::ThreeOfAkind,
                (3, 2) => HandType::TwoPair,
                (4, _) => HandType::Pair,
                _ => HandType::HighCard,
            }
        }
    }

    fn parse_line(l: String) -> (Hand, usize) {
        let mut parts = l.split(" ");
        let hand: Hand = parts.next().unwrap().into();
        let bid = parts.next().unwrap().parse().unwrap();

        (hand, bid)
    }

    pub(crate) fn part2() {
        let lines = read_lines("day07/input").unwrap();
        let mut bids = lines
            .map(|line| parse_line(line.unwrap()))
            .collect::<Vec<_>>();
        bids.sort();
        let winnings = bids
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum::<usize>();

        println!("{}", winnings);
    }
}

fn main() {
    part1::part1();
    part2::part2();
}
