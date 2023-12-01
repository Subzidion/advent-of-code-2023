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
        let mut current_string = "".to_owned();
        for c in line.chars() {
            match c.to_digit(10) {
                Some(d) => {
                    current_string = "".to_owned();
                    if first == 0 {
                        first = d;
                    }
                    second = d;
                }
                None => {
                    current_string.push(c);
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
