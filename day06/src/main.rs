use std::{fs, io, iter::zip, str::FromStr};

use common::read_lines;

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn ways_to_beat(&self) -> Vec<usize> {
        (1..self.time)
            .map(|t| (self.time - t) * t)
            .filter(|&distance| distance > self.record_distance)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Records {
    races: Vec<Race>,
}

impl FromStr for Records {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let times = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|p| !p.is_empty())
            .skip(1)
            .map(|t| t.parse::<usize>().unwrap());

        let distances = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|p| !p.is_empty())
            .skip(1)
            .map(|t| t.parse::<usize>().unwrap());

        let races = zip(times, distances)
            .map(|(time, record_distance)| Race {
                time,
                record_distance,
            })
            .collect();

        Ok(Records { races })
    }
}

fn part1() {
    let lines = fs::read_to_string("day06/input").unwrap();
    let r = lines.parse::<Records>().unwrap();
    let p = r
        .races
        .iter()
        .map(|race| race.ways_to_beat().len())
        .product::<usize>();
    println!("{:?}", p);
}

fn main() {
    part1();
}
