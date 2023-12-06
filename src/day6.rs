#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let race_1 = (42, 284);
    let race_2 = (68, 1005);
    let race_3 = (69, 1122);
    let race_4 = (85, 1341);
    let races: Vec<(u32, u32)> = vec![race_1, race_2, race_3, race_4];

    let mut total = 1;

    for race in races {
        let mut race_wins: u32 = 0;
        for hold_time in 0..race.0 {
            let time_left = race.0 - hold_time;
            let distance = hold_time * time_left;

            if distance > race.1 {
                race_wins += 1;
            }
        }

        total *= race_wins;
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let race_1: (u64, u64) = (42686985, 284100511221341);
    let races: Vec<(u64, u64)> = vec![race_1];

    let mut total = 1;

    for race in races {
        let mut race_wins: u32 = 0;
        for hold_time in 0..race.0 {
            let time_left = race.0 - hold_time;
            let distance = hold_time * time_left;

            if distance > race.1 {
                race_wins += 1;
            }
        }

        total *= race_wins;
    }

    total
}
