use std::collections::HashMap;

pub struct Game {
    id: u32,
    hands: Vec<Hand>,
}

pub struct Hand {
    grab: HashMap<String, u32>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    // Convert each Line into a Game and Collect into a Vector
    input
        .lines()
        .map(|line| {
            // Split the "Game ##: ..." into "Game ##" and "...Set of Hands"
            let mut game_line = line.split(':');

            // Increment the Split to get the "Game ##"
            // create new Split of "Game ##" into "Game" and "##"
            // increment the Split in reverse to get "##", parse the integer
            let game_number: u32 = game_line
                .next()
                .unwrap()
                .rsplit(' ')
                .next()
                .unwrap()
                .parse()
                .unwrap();

            // Increment the Split to get the "...Set of Hands"
            // Split the set of Hands on the semi-colon.
            let hands_string = game_line.next().unwrap().split(';');

            // Parse the split of Hand Strings and collect into a Vector of Hands.
            let hands: Vec<Hand> = hands_string
                // Iterate over each hand and collect the "Grabs" as a set of Colors
                .map(|hand| {
                    // Create a split on the comma to parse each color individually
                    // Then collect that into a HashMap for the individual Grab
                    let grab: HashMap<String, u32> = hand
                        .split(',')
                        .map(|colors_string| {
                            // Split the color by number and color
                            let mut color_string = colors_string.trim().split(' ');
                            // parse the count
                            let color_count: u32 = color_string.next().unwrap().parse().unwrap();
                            // parse the color
                            let color: String = color_string.next().unwrap().parse().unwrap();

                            // Create the Tuple that will be collected into a HashMap
                            (color, color_count)
                        })
                        .collect();

                    Hand { grab: grab }
                })
                .collect();

            // Create the Game with its Line Number and the individual Hands
            Game {
                id: game_number,
                hands: hands,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &[Game]) -> u32 {
    let mut total = 0;
    games.iter().for_each(|game| {
        let mut should_add = true;
        game.hands.iter().for_each(|hand| {
            if hand.grab.get("red").unwrap_or(&0) > &12 {
                should_add = false;
            }

            if hand.grab.get("green").unwrap_or(&0) > &13 {
                should_add = false;
            }

            if hand.grab.get("blue").unwrap_or(&0) > &14 {
                should_add = false;
            }
        });

        if should_add {
            total += game.id;
        }
    });

    total
}

#[aoc(day2, part2)]
pub fn part2(games: &[Game]) -> u32 {
    let mut total = 0;

    games.iter().for_each(|game| {
        let mut min_red_needed = 0;
        let mut min_green_needed = 0;
        let mut min_blue_needed = 0;

        game.hands.iter().for_each(|hand| {
            let red = hand.grab.get("red").unwrap_or(&0);
            if red > &min_red_needed {
                min_red_needed = *red;
            }

            let green = hand.grab.get("green").unwrap_or(&0);
            if green > &min_green_needed {
                min_green_needed = *green;
            }

            let blue = hand.grab.get("blue").unwrap_or(&0);
            if blue > &min_blue_needed {
                min_blue_needed = *blue;
            }
        });

        total += min_red_needed * min_green_needed * min_blue_needed
    });

    total
}
