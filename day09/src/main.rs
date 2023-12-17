use std::iter::zip;

use common::read_lines;

fn derive(values: Vec<i64>) -> Vec<i64> {
    let first = values.iter();
    let second = values.iter().skip(1);
    zip(first, second).map(|(a, b)| b - a).collect()
}

fn extrapolate(in_values: &Vec<i64>) -> i64 {
    let mut values = in_values.clone();
    let mut last_vals = vec![*values.last().unwrap()];
    while values.iter().any(|x| *x != values[0]) {
        values = derive(values);
        last_vals.push(*values.last().unwrap());
    }

    // values is now a constant value
    let new_val = last_vals.iter().sum::<i64>();
    println!("extrapolate for: {:?} {}", in_values, new_val);
    new_val
}

fn extrapolate_back(in_values: &Vec<i64>) -> i64 {
    let mut values = in_values.clone();
    let mut first_values = vec![values[0]];
    while values.iter().any(|x| *x != values[0]) {
        values = derive(values);
        first_values.push(values[0]);
    }

    // now run on the first values to get the extrapolated ones:
    let mut last_derived = 0;
    first_values.reverse();
    for v in first_values.iter() {
        last_derived = v - last_derived;
    }

    last_derived
}

fn part1() {
    let mut sum_extrapolated = 0;
    for line in read_lines("day09/input").unwrap() {
        let values = line
            .unwrap()
            .split(" ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        sum_extrapolated += extrapolate(&values);
    }
    println!("{}", sum_extrapolated);
}

fn part2() {
    let mut sum_extrapolated = 0;
    for line in read_lines("day09/input").unwrap() {
        let values = line
            .unwrap()
            .split(" ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        sum_extrapolated += extrapolate_back(&values);
    }
    println!("total: {}", sum_extrapolated);
}

fn main() {
    part1();
    part2();
}
