use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
};

use common::read_lines;

#[derive(Debug, Clone)]
struct Range {
    dest_range_start: usize,
    src_range_start: usize,
    range_size: usize,
}
impl Range {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(" ");
        let dest_range_start = parts.next().unwrap().parse().unwrap();
        let src_range_start = parts.next().unwrap().parse().unwrap();
        let range_size = parts.next().unwrap().parse().unwrap();

        Range {
            dest_range_start,
            src_range_start,
            range_size,
        }
    }
}
#[derive(Debug, Clone)]
struct RangeMap(Vec<Range>);

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl RangeMap {
    fn parse<T: BufRead>(peekable_lines: &mut Peekable<Lines<T>>) -> Self {
        let mut map = RangeMap(Vec::new());
        // read empty line
        assert_eq!(peekable_lines.next().unwrap().expect("an empty line"), "");
        // read the title line
        assert!(peekable_lines
            .next()
            .unwrap()
            .expect("map title")
            .ends_with("map:"));

        loop {
            let peek_val = peekable_lines.peek();
            match peek_val {
                None => {
                    return map;
                }
                Some(Ok(line)) => {
                    if line == "" {
                        return map;
                    } else {
                        map.0.push(Range::parse(line));
                        peekable_lines.next();
                    }
                }
                Some(Err(_)) => panic!("An error occurred"),
            }
        }
    }

    fn get_dest(&self, source: usize) -> usize {
        // iterate on all ranges until a dest is found - otherwise, it is mapped directly
        for r in self.0.iter() {
            if source >= r.src_range_start && source < r.src_range_start + r.range_size {
                return r.dest_range_start + (source - r.src_range_start);
            }
        }

        return source;
    }
}

impl Almanac {
    fn parse<T: BufRead>(mut lines: Lines<T>) -> Self {
        // parse the seeds line
        let seed_line = lines.next().unwrap().unwrap();
        let seeds = seed_line
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .map(|seed_str| seed_str.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut peekable_lines = lines.peekable();
        let seed_to_soil = RangeMap::parse(&mut peekable_lines);
        let soil_to_fertilizer = RangeMap::parse(&mut peekable_lines);
        let fertilizer_to_water = RangeMap::parse(&mut peekable_lines);
        let water_to_light = RangeMap::parse(&mut peekable_lines);
        let light_to_temperature = RangeMap::parse(&mut peekable_lines);
        let temperature_to_humidity = RangeMap::parse(&mut peekable_lines);
        let humidity_to_location = RangeMap::parse(&mut peekable_lines);

        Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn location_for_seed(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get_dest(seed);
        let fertilizer = self.soil_to_fertilizer.get_dest(soil);
        let water = self.fertilizer_to_water.get_dest(fertilizer);
        let light = self.water_to_light.get_dest(water);
        let temp = self.light_to_temperature.get_dest(light);
        let humidity = self.temperature_to_humidity.get_dest(temp);
        return self.humidity_to_location.get_dest(humidity);
    }
}

fn part1() {
    let lines: Lines<BufReader<File>> = read_lines("day05/input").unwrap();
    let almanac = Almanac::parse(lines);
    for seed in almanac.seeds.iter() {
        println!("{:?} -> {:?}", seed, almanac.location_for_seed(*seed));
    }

    let location = almanac
        .seeds
        .iter()
        .map(|seed| almanac.location_for_seed(*seed))
        .min()
        .unwrap();
    println!("{:?}", location);
}

fn main() {
    part1();
}
