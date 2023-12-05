use std::collections::HashSet;

#[derive(Clone)]
pub struct Card {
    id: u32,
    overlap: u32,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    // Convert each Line into a Card and Collect into a Vector
    input
        .lines()
        .map(|line| {
            // Split the "Card ##: ..." into "Card ##" and "...Set of Numbers"
            let mut card_line = line.split(':');

            // Increment the Split to get the "Card ##"
            // create new Split of "Card ##" into "Card" and "##"
            // increment the Split in reverse to get "##", parse the integer
            let card_number: u32 = card_line
                .next()
                .unwrap()
                .rsplit(' ')
                .next()
                .unwrap()
                .parse()
                .unwrap();

            // Increment the Split to get the "...Set of Numbers"
            // Split the set of Numbers on the pipe.
            let mut numbers_string = card_line.next().unwrap().split('|');

            // Collect the winning numbers into a Set
            let winning_numbers: HashSet<u32> = numbers_string
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .into_iter()
                .map(|c| return c.parse().unwrap())
                .collect();

            // Collect the present numbers into a Set
            let present_numbers: HashSet<u32> = numbers_string
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .into_iter()
                .map(|c| return c.parse().unwrap())
                .collect();

            // Create the Card
            Card {
                id: card_number,
                overlap: winning_numbers
                    .intersection(&present_numbers)
                    .count()
                    .try_into()
                    .unwrap(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(cards: &[Card]) -> u32 {
    // 2 to the power of the overlap - 1 * 1 is our score for the card.
    // Base is necessary in Rust to do exponents.
    let base: u32 = 2;

    cards
        .iter()
        .fold(0, |acc, card| acc + (1 * base.pow(card.overlap - 1)))
}

#[aoc(day4, part2)]
pub fn part2(cards: &[Card]) -> u32 {
    cards
        .iter()
        .fold(0, |acc, card| acc + process_card(card, cards))
}

fn process_card(card: &Card, cards: &[Card]) -> u32 {
    let mut total = 0;

    for i in card.id..(card.id + card.overlap) {
        let index: usize = i.try_into().unwrap();
        total += process_card(&cards[index], cards);
    }

    total + 1
}
