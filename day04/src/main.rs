use core::num;
use std::collections::{HashMap, HashSet};

use common::read_lines;
use regex::Regex;

fn score_card(card: String) -> usize {
    let re = Regex::new(r"Card (?<id>[0-9]*).*: (?<winning>.*)\|(?<have>.*)").unwrap();
    let cap = re.captures(&card).unwrap();
    let winning = cap["winning"]
        .trim()
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let have = cap["have"]
        .trim()
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    let num_matches = winning.intersection(&have).count();
    match num_matches {
        0 => 0,
        _ => 2usize.pow((num_matches - 1) as u32),
    }
}

struct Card {
    id: u32,
    won_cards: Vec<u32>,
}

fn parse_card(card_str: &str) -> Card {
    let re = Regex::new(r"Card +(?<id>[0-9]+).*: (?<winning>.*)\|(?<have>.*)").unwrap();
    let cap = re.captures(&card_str).unwrap();
    let id: u32 = cap["id"].trim().split(" ").last().unwrap().parse().unwrap();
    let winning = cap["winning"]
        .trim()
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let have = cap["have"]
        .trim()
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    let num_matches = winning.intersection(&have).count() as u32;
    Card {
        id,
        won_cards: (id + 1..=id + num_matches).collect(),
    }
}

fn part1() {
    let lines = read_lines("day04/input").unwrap();
    let total_score: usize = lines.map(|l| score_card(l.unwrap())).sum();
    println!("{}", total_score);
}

fn part2() {
    let lines = read_lines("day04/input").unwrap();
    let cards = lines.map(|l| parse_card(&l.unwrap())).collect::<Vec<_>>();
    let mut card_counts = HashMap::<u32, u32>::new();

    for card in cards.iter() {
        // always add the card we just found
        let curr_count = card_counts.entry(card.id).or_insert(0);
        *curr_count += 1u32;

        // create a copy so we don't have 2 mutable references
        let new_count = *curr_count;

        // since we have multiple copies of this card, we add multiple instances of each card
        for won_card in &card.won_cards {
            *card_counts.entry(*won_card).or_insert(0) += new_count;
        }
    }

    println!("{}", card_counts.values().sum::<u32>());
}

fn main() {
    part1();
    part2();
}
