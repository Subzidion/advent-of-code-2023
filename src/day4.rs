use std::collections::HashSet;

pub struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    present_numbers: HashSet<u32>,
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
                winning_numbers: winning_numbers,
                present_numbers: present_numbers,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(cards: &[Card]) -> u32 {
    let mut total = 0;
    // 2 to the power of the overlap - 1 * 1 is our score for the card.
    // Base is necessary in Rust to do exponents.
    let base: u32 = 2;

    cards.iter().for_each(|card| {
        // Calculate the intersection of the two sets to get the number of overlaps.
        let overlap: u32 = card
            .winning_numbers
            .intersection(&card.present_numbers)
            .count()
            .try_into()
            .unwrap();

        // Calculate the score of the Card.
        let score = 1 * base.pow(overlap - 1);

        total += score;
    });

    total
}
