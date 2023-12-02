use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut second = 0;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(d) => {
                    if first == 0 {
                        first = d;
                    }
                    second = d;
                }
                None => {}
            };
        }
        total += first * 10 + second;
    }

    total
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut numbers: HashMap<&str, u32> = HashMap::new();
    numbers.insert("one", 1);
    numbers.insert("two", 2);
    numbers.insert("three", 3);
    numbers.insert("four", 4);
    numbers.insert("five", 5);
    numbers.insert("six", 6);
    numbers.insert("seven", 7);
    numbers.insert("eight", 8);
    numbers.insert("nine", 9);

    let mut total = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut second = 0;

        // Start the window at the far left
        let mut start_index = 0;
        // Set the end of the window
        let mut end_index;

        if line.len() < 3 {
            // If the string is short, the window should be the entire string.
            end_index = line.len() - 1;
        } else {
            // Start with the smallest potential number length (one == 3 characters)
            // This is for strings that start with short numbers.
            // Otherwise, the algorithm will chop from the left and never see it with a bigger window.
            end_index = 2;
        }

        for (i, c) in line.chars().enumerate() {
            // Only _after_ the first pass-though
            // Slide the right side of the window by 1, if there's room.
            if i != 0 && end_index < line.len() {
                end_index += 1;
            }
            // We want to start sliding the window when the end_index is greater than 5,
            // the maximum window size necessary to find a valid number (seven == 5 characters)
            if end_index > 5 {
                start_index += 1;
            }

            // Along the way, short-circuit if we find any characters.
            // TODO: We could probably aggressively re-index here after the found digit.
            match c.to_digit(10) {
                // Valid Digit
                Some(d) => {
                    if first == 0 {
                        first = d;
                    }
                    second = d;
                }
                // Non-digit character
                None => {
                    // Get your current window as a slice
                    let current_string = &line[start_index..end_index];

                    // Because we've shifted the window to the right by one,
                    // We need to check again that no new number has been created from the entire string
                    // or that a new smaller number has not appeared.
                    //
                    // This is done by checking the entire string and seeing if it's a valid number.
                    // Then checking for new smaller numbers by dwindling the size of the window.
                    // TODO: Could probably be aggressive with number lengths here. No need for checking < 3 or > 5.
                    for i in 0..current_string.len() {
                        let check_string = &current_string[i..];
                        match numbers.get(check_string) {
                            Some(d) => {
                                if first == 0 {
                                    first = *d;
                                }
                                second = *d;
                                break;
                            }
                            None => {}
                        }
                    }
                }
            };
        }
        total += first * 10 + second;
    }

    total
}
