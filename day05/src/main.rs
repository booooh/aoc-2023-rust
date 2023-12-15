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
                    map.0.sort_by(|a, b| {
                        a.dest_range_start.partial_cmp(&b.dest_range_start).unwrap()
                    });
                    return map;
                }
                Some(Ok(line)) => {
                    if line == "" {
                        map.0.sort_by(|a, b| {
                            a.dest_range_start.partial_cmp(&b.dest_range_start).unwrap()
                        });
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

    /// returns the subset of the target range
    /*

    example:

    humidity->location

    100 1000 300 (map humidity 1000-1300 to location 100-400)


    temperature->humidity

    800  9800 400 (map temperature 9800-10200 to humidity 800-1200)
    1200 11000 50 (map temperature 11000-11050 to humidity 1200-1250)
    ...           (implicitly map temperature 1250-1300 to humidity 1250-1300)


    given humidity range (1000-1300) ->
    return temperatre ranges:
        10000-10200 (1000-1200)
        11000-11050 (1200-1250)
        1250-1300   (1250-1300)
    */
    fn get_subranges(&self, target_dest_range: &Range) -> Vec<Range> {
        // the destination range may span several ranges in this map
        let mut sources = Vec::<Range>::new();
        let target_start = target_dest_range.dest_range_start;
        let target_end = target_dest_range.dest_range_start + target_dest_range.range_size;

        // vector is sorted by target range, so if there are any gaps, we need to use implicit mapping
        for r in self.0.iter() {
            // check if there's an overlap of the target range and this range
            let curr_end = r.dest_range_start + r.range_size;
            if (r.dest_range_start >= target_start && r.dest_range_start < target_end)
                || (target_start >= r.dest_range_start && target_start < curr_end)
            {
                let overlap_start = std::cmp::max(target_start, r.dest_range_start);
                let overlap_end = std::cmp::min(target_end, curr_end);
                let overlap_size = overlap_end - overlap_start;

                // check if we need to add an implict range before this range
                if let Some(prev) = sources.last() {
                    if !(prev.dest_range_start + prev.range_size == overlap_start) {
                        sources.push(Range {
                            dest_range_start: prev.dest_range_start + prev.range_size,
                            src_range_start: prev.dest_range_start + prev.range_size,
                            range_size: overlap_start - (prev.dest_range_start + prev.range_size),
                        })
                    }
                } else {
                    // if this is the first item, and there's a gap, add the implicit mapping
                    if overlap_start > target_start {
                        sources.push(Range {
                            dest_range_start: target_start,
                            src_range_start: target_start,
                            range_size: overlap_start - target_start,
                        })
                    }
                }

                // add the subrange to the vector
                sources.push(Range {
                    dest_range_start: overlap_start,
                    src_range_start: r.src_range_start + (overlap_start - r.dest_range_start),
                    range_size: overlap_size,
                })
            }
        }

        // if there's no mapping for the end, add the implict mapping until target_end
        if let Some(last) = sources.last() {
            if last.dest_range_start + last.range_size < target_end {
                sources.push(Range {
                    dest_range_start: last.dest_range_start + last.range_size,
                    src_range_start: last.dest_range_start + last.range_size,
                    range_size: target_end - (last.dest_range_start + last.range_size),
                })
            }
        } else {
            // there's no overlap, just add the entire range
            sources.push(Range {
                dest_range_start: target_dest_range.dest_range_start,
                src_range_start: target_dest_range.dest_range_start,
                range_size: target_dest_range.range_size,
            })
        }

        return sources;
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

    fn get_locations_for_range(&self, r: std::ops::Range<usize>) -> Vec<usize> {
        println!("going to look for the locations of range {:?}", r);
        r.into_iter()
            .map(|seed| self.location_for_seed(seed))
            .collect()
    }

    fn seed_ranges_for_location_range(&self, location_range: &Range) -> Vec<Range> {
        let mut res = Vec::new();
        // expect this to be a vector with one element (since we're taking the range directly from there)
        let humidity_ranges = self.humidity_to_location.get_subranges(location_range);

        for h_range in humidity_ranges.iter() {
            // find ranges that can map temperature to the humidity ranges (invariant: each of the final seed ranges can lead to the location range passed in)
            let desired_temp_range = Range {
                dest_range_start: h_range.src_range_start,
                src_range_start: h_range.src_range_start, // not used
                range_size: h_range.range_size,
            };
            let temp_ranges = self
                .temperature_to_humidity
                .get_subranges(&desired_temp_range);
            for t_range in temp_ranges.iter() {
                let desired_light_range = Range {
                    dest_range_start: t_range.src_range_start,
                    src_range_start: t_range.src_range_start, // not used
                    range_size: t_range.range_size,
                };
                let light_ranges = self
                    .light_to_temperature
                    .get_subranges(&desired_light_range);
                for l_range in light_ranges.iter() {
                    let desired_water_range = Range {
                        dest_range_start: l_range.src_range_start,
                        src_range_start: l_range.src_range_start, //not used
                        range_size: l_range.range_size,
                    };
                    let water_ranges = self.water_to_light.get_subranges(&desired_water_range);
                    for w_range in water_ranges.iter() {
                        let desired_fert_range = Range {
                            dest_range_start: w_range.src_range_start,
                            src_range_start: w_range.src_range_start, // not used
                            range_size: w_range.range_size,
                        };
                        let fert_ranges =
                            self.fertilizer_to_water.get_subranges(&desired_fert_range);
                        for f_range in fert_ranges.iter() {
                            let desired_soil_range = Range {
                                dest_range_start: f_range.src_range_start,
                                src_range_start: f_range.src_range_start, // not used
                                range_size: f_range.range_size,
                            };
                            let soil_ranges =
                                self.soil_to_fertilizer.get_subranges(&desired_soil_range);
                            for soil_range in soil_ranges.iter() {
                                let desired_seed_range = Range {
                                    dest_range_start: soil_range.src_range_start,
                                    src_range_start: soil_range.src_range_start, //not used
                                    range_size: soil_range.range_size,
                                };
                                res.extend(self.seed_to_soil.get_subranges(&desired_seed_range));
                            }
                        }
                    }
                }
            }
        }
        return res;
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

fn part2() {
    let lines: Lines<BufReader<File>> = read_lines("day05/input").unwrap();
    let almanac = Almanac::parse(lines);

    let mut ranges = almanac.seeds[..]
        .chunks(2)
        .map(|seed_range| seed_range[0]..(seed_range[0] + seed_range[1]))
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let smallest_loc = almanac.humidity_to_location.0.first().unwrap();
    println!("smallest location-range{:?}", smallest_loc);

    let possible_seed_ranges = almanac.seed_ranges_for_location_range(smallest_loc);
    println!("options for seed-ranges: {:?}", possible_seed_ranges);
    println!(
        "number of seeds overall: {:?}",
        possible_seed_ranges
            .iter()
            .map(|r| r.range_size)
            .sum::<usize>()
    );

    for p in possible_seed_ranges.iter() {
        let p_start = p.src_range_start;
        let p_end = p.src_range_start + p.range_size;
        let pr = p_start..p_end;
        for sr in ranges.iter() {
            if sr.contains(&pr.start) || pr.contains(&sr.start) {
                let sub_r = std::cmp::max(pr.start, sr.start)..std::cmp::min(pr.end, sr.end);
                println!("Found a possible sub range! {:?} - {:?}", sub_r, p);
                println!(
                    "  location for seed values: {:?} {:?}",
                    almanac.location_for_seed(sub_r.start),
                    almanac.location_for_seed(sub_r.end)
                )
            }
        }
    }

    // find if any of the seed-ranges listed match the possible-seed ranges

    // let locations = ranges
    //     .iter()
    //     .map(|r| almanac.get_locations_for_range(r.clone()))
    //     .flatten();
    // println!("{:?}", locations.min());
}

fn main() {
    part1();
    part2();
}
