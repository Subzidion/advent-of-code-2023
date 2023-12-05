#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    // Iterate over the lines (ignore empty lines)
    let mut lines = input.lines().filter(|line| !line.is_empty());

    // The first line is the Seed line.
    let mut seed_string = lines.next().unwrap().split(':');
    // Skip the "seeds:" portion of the string.
    seed_string.next();

    // Parse the seeds from the first line.
    let mut seeds: Vec<(i64, bool)> = seed_string
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .into_iter()
        .map(|seed| (seed.parse().unwrap(), false))
        .collect();

    for line in lines {
        if line.contains(":") {
            // This indicates the beginning of a new map, so we reset the visitors.
            for seed in seeds.iter_mut() {
                seed.1 = false;
            }
            continue;
        } else {
            // Parse a line of the map.
            let converter: Vec<i64> = line
                .split_ascii_whitespace()
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect();

            // Parse the range starts, ranges, and differences.
            let destination_range_start = *converter.get(0).unwrap();
            let source_range_start = *converter.get(1).unwrap();
            let range = *converter.get(2).unwrap();
            let difference: i64 = (destination_range_start - source_range_start)
                .try_into()
                .unwrap();

            // Iterate through seeds and see if they are to be processed by this converter line.
            // If the seed has already been visited, the flag needs to be set to not visit it again this map.
            for index in 0..seeds.len() {
                if !seeds[index].1
                    && seeds[index].0 >= source_range_start
                    && (seeds[index].0 - range) <= source_range_start
                {
                    // Update the seed and mark it as visited for this map.
                    seeds[index].0 = seeds[index].0 + difference;
                    seeds[index].1 = true;
                }
            }
        }
    }

    // Find the minimum seed and return.
    let mut min = seeds.get(0).unwrap().0;
    for seed in seeds {
        if seed.0 < min {
            min = seed.0;
        }
    }

    min
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    // Iterate over the lines (ignore empty lines)
    let mut lines = input.lines().filter(|line| !line.is_empty());

    // The first line is the Seed line.
    let mut seed_string = lines.next().unwrap().split(':');
    // Skip the "seeds:" portion of the string.
    seed_string.next();

    // Parse the seeds range from the first line.
    let seeds_range: Vec<i64> = seed_string
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .into_iter()
        .map(|seed| seed.parse().unwrap())
        .collect();

    // Convert the seeds range into a set of seeds
    let mut seeds: Vec<(i64, bool)> = Vec::new();

    // Generate the seeds
    for index in (0..seeds_range.len()).step_by(2) {
        let starting_seed = *seeds_range.get(index).unwrap();
        let ending_seed = starting_seed + *seeds_range.get(index + 1).unwrap();
        for seed in starting_seed..ending_seed {
            seeds.push((seed, false));
        }
    }

    for line in lines {
        if line.contains(":") {
            // This indicates the beginning of a new map, so we reset the visitors.
            for seed in seeds.iter_mut() {
                seed.1 = false;
            }
            continue;
        } else {
            // Parse a line of the map.
            let converter: Vec<i64> = line
                .split_ascii_whitespace()
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect();

            // Parse the range starts, ranges, and differences.
            let destination_range_start = *converter.get(0).unwrap();
            let source_range_start = *converter.get(1).unwrap();
            let range = *converter.get(2).unwrap();
            let difference: i64 = (destination_range_start - source_range_start)
                .try_into()
                .unwrap();

            // Iterate through seeds and see if they are to be processed by this converter line.
            // If the seed has already been visited, the flag needs to be set to not visit it again this map.
            for index in 0..seeds.len() {
                if !seeds[index].1
                    && seeds[index].0 >= source_range_start
                    && (seeds[index].0 - range) <= source_range_start
                {
                    // Update the seed and mark it as visited for this map.
                    seeds[index].0 = seeds[index].0 + difference;
                    seeds[index].1 = true;
                }
            }
        }
    }

    // Find the minimum seed and return.
    let mut min = seeds.get(0).unwrap().0;
    for seed in seeds {
        if seed.0 < min {
            min = seed.0;
        }
    }

    min
}
