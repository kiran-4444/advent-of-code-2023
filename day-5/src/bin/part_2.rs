use nom::{bytes::complete::tag, IResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<i64>, Range<i64>)>,
}

impl SeedMap {
    fn calculate(&self, source: i64) -> i64 {
        // dbg!(source);
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        // dbg!(source, valid_mapping);

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output.ok().unwrap().1);
}

fn part_2(input: &str) -> IResult<&str, String> {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    let (seeds, _) = tag("seeds: ")(blocks[0])?;
    let seeds_vec = seeds
        .split(" ")
        .map(|digit| digit.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mapping_blocks = blocks
        .iter()
        .skip(1)
        .map(|line| line.split("\n").skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut map_vecs: Vec<Vec<Vec<i64>>> = Vec::new();

    for (_, blocks) in mapping_blocks.iter().enumerate() {
        let mut temp_map_vec: Vec<Vec<i64>> = Vec::new();
        for block in blocks {
            let splitted_block = block
                .split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let source = splitted_block[1];
            let destination = splitted_block[0];
            let rng = splitted_block[2];
            temp_map_vec.push(vec![source, destination, rng]);
        }

        map_vecs.push(temp_map_vec);
    }

    // dbg!(&map_vecs);

    let mut seed_maps: Vec<SeedMap> = Vec::new();

    for i in map_vecs {
        let mut range_vecs: Vec<(Range<i64>, Range<i64>)> = Vec::new();
        for j in i {
            let range_1 = Range {
                start: j[0],
                end: j[0] + j[2],
            };
            let range_2 = Range {
                start: j[1],
                end: j[1] + j[2],
            };
            range_vecs.push((range_1, range_2));
        }
        let seed_map: SeedMap = SeedMap {
            mappings: range_vecs,
        };
        seed_maps.push(seed_map);
    }

    // dbg!(&seed_maps);

    let seed_ranges = seeds_vec
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .collect::<Vec<Range<i64>>>();

    // dbg!(&seed_ranges);

    // let locations = seeds_vec
    //     .iter()
    //     .map(|seed| {
    //         seed_maps
    //             .iter()
    //             .fold(*seed, |seed, map| map.calculate(seed))
    //     })
    //     .collect::<Vec<i64>>();

    let minimum_location = seed_ranges
        .into_par_iter()
        // .progress_count(count)
        .flat_map(|range| range.clone())
        .map(|seed| seed_maps.iter().fold(seed, |seed, map| map.calculate(seed)))
        .min();

    Ok((input, minimum_location.unwrap().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./input.txt");
        let output = part_2(input).unwrap();
        assert_eq!(output.1, "46");
    }
}
