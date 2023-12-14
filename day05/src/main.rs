use core::panic;
use std::{
    borrow::BorrowMut,
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
        // let next_val = peekable_lines.peek();

        println!("going to read a map\n\n");
        // while the next value is not the end of the file, and is not an empty line
        // let next_val = peekable_lines.peek();
        // while next_val.is_some() && !next_val.unwrap().as_ref().unwrap().is_empty() {
        //     map.0
        //         .push(Range::parse(next_val.unwrap().as_ref().unwrap()));
        //     _ = peekable_lines.next(); // advance the iterator so we consume what we just tested
        // }
        loop {
            let peek_val = peekable_lines.peek();
            match peek_val {
                None => {
                    return map;
                }
                Some(Ok(line)) => {
                    if line == "" {
                        println!("Got an empty line!");
                        return map;
                    } else {
                        map.0.push(Range::parse(line));
                        peekable_lines.next();
                    }
                }
                Some(Err(_)) => panic!("An error occurred"),
            }
        }
        map
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
}

fn part1() {
    let lines: Lines<BufReader<File>> = read_lines("day05/input").unwrap();
    let almanac = Almanac::parse(lines);
    println!("{:?}", almanac);
}

fn main() {
    part1();
    println!("Hello, world!");
}
