use std::{
    fs,
    io::{self, Error},
    iter::zip,
    str::FromStr,
};

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

impl FromStr for Race {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let time = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|p| !p.is_empty())
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|p| !p.is_empty())
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        return Ok(Self {
            record_distance: distance,
            time: time,
        });
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

fn part2() {
    let lines = fs::read_to_string("day06/input").unwrap();
    let r = lines.parse::<Race>().unwrap();
    println!("{:?}", r.ways_to_beat().len());
}

fn main() {
    part1();
    part2();
}
