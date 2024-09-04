use crate::utils::read_file;
use std::collections::HashMap;

use std::io;

pub fn run(input_path: &str) -> Result<u64, io::Error> {
    // Calling `.expect` would automatically panic and crash the program
    // using the `?` syntax, we will either return the contents in the success case
    // or the Error will be passed on directly to the calling function
    let content = read_file(input_path)?;

    // parse string into a list of Cards
    let mut maps: HashMap<String, Map> = HashMap::new();

    let answer = parse_input(content, &mut maps);

    Ok(answer)
}

struct Map {
    // name: String,
    source_range_starts: Vec<u64>,
    dest_range_starts: Vec<u64>,
    range_lengths: Vec<u64>,
}

fn parse_input(content: String, maps: &mut HashMap<String, Map>) -> u64 {
    let splits: Vec<&str> = content.split("\n\n").collect();
    println!("Number of newlines: {:?}", splits.len());

    // first line is the number of seeds that need to be sown.
    let seeds = splits[0].strip_prefix("seeds: ").unwrap().split(" ");
    let mut int_seeds: Vec<u64> = Vec::new();
    for seed in seeds {
        int_seeds.push(seed.parse().expect(&format!("Failed to parse {:?}", seed)))
    }

    // the blocks after that each have a mapping
    // use usize for indexing and i32 for actual integer computations.
    for i in 1..splits.len() {
        let mapping: Vec<&str> = splits[i].split("\n").collect();
        // first line in a block is the mapping name
        let map_name: &str = mapping[0].split(" ").collect::<Vec<&str>>()[0];
        println!("\nMap: {map_name}\n{:?}", "=".repeat(40));

        let mut source_start: Vec<u64> = Vec::new();
        let mut destination_start: Vec<u64> = Vec::new();
        let mut length: Vec<u64> = Vec::new();

        // the following lines are each
        for j in 1..mapping.len() {
            let mut map_section: Vec<&str> = mapping[j].split(" ").collect();

            destination_start.push(map_section[0].parse().unwrap());
            source_start.push(map_section[1].parse().unwrap());
            length.push(map_section[2].parse().unwrap());
        }

        // order according to the source
        let mut zipped: Vec<(&u64, &u64, &u64)> = source_start
            .iter()
            .zip(destination_start.iter())
            .zip(length.iter())
            .map(|((a, b), c)| (a, b, c))
            .collect();

        // .zip(vec3.iter()): This then zips the resulting iterator with vec3, creating tuples of tuples ((1st from vec1, 1st from vec2), 1st from vec3).
        // .map(|((a, b), c)| (*a, *b, *c)): This map is used to transform the nested tuples into flat 3-tuples. It de-references the values since iter() creates iterators over references.
        // collect(): Finally, collect gathers all the 3-tuples into a Vec.

        // sort by uses a closure to destermine which value to use as a key.
        zipped.sort_by_key(|&(key, _, _)| key);

        let mut starts_sorted: Vec<u64> = Vec::new();
        let mut destinations_sorted: Vec<u64> = Vec::new();
        let mut lengths_sorted: Vec<u64> = Vec::new();

        println!("source, dest, len\n");

        for (start, dest, len) in zipped {
            starts_sorted.push(*start);
            destinations_sorted.push(*dest);
            lengths_sorted.push(*len);
            println!("{start} {dest} {len}");
        }

        let curr_map = Map {
            // name:.to_string(),
            source_range_starts: starts_sorted,
            dest_range_starts: destinations_sorted,
            range_lengths: lengths_sorted,
        };
        maps.insert(map_name.to_string(), curr_map);
    }

    let stages = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    // find the location for each of the seeds
    let mut locations: Vec<u64> = Vec::new();

    for seed in int_seeds {
        println!("\nRunning for seed: {:?}\n{:?}", seed, "=".repeat(50));
        // let mut curr_position = 0;
        let mut stage_value = seed;
        for &stage in &stages {
            let curr_map = maps.get(stage).unwrap();

            // find the correct index start

            for i in 0..curr_map.source_range_starts.len() {
                if stage_value >= curr_map.source_range_starts[i]
                    && stage_value < curr_map.source_range_starts[i] + curr_map.range_lengths[i]
                {
                    let diff = stage_value.abs_diff(curr_map.source_range_starts[i]);
                    println!("inside range with diff: {diff} stage value: {stage_value} source range start: {:?}",curr_map.source_range_starts[i]);
                    stage_value = curr_map.dest_range_starts[i] + diff;
                    break;
                }
            }
            println!("stage: {:?} value: {:?}", stage, stage_value);
            // curr_position = stage_value;
        }

        println!("final: seed: {:?} location: {:?}", seed, stage_value);
        locations.push(stage_value);
    }

    locations.sort();
    println!("smallest location: {:?}", locations[0]);
    locations[0]
}
