use std::collections::HashMap;

use common::read_lines;

const M: usize = 140;
const N: usize = 140;

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct SchemaItem {
    start: Coord,
    end: Coord,
    value: u32,
}

impl SchemaItem {
    fn get_neighbor_coords(&self) -> Vec<Coord> {
        let mut neighbors = Vec::new();
        let start_x = match self.start.x {
            0 => 0,
            _ => self.start.x - 1,
        };
        let end_x = match self.end.x + 1 {
            M => M - 1,
            _ => self.end.x + 1,
        };
        if self.start.y > 0 {
            for x in start_x..end_x + 1 {
                neighbors.push(Coord {
                    x,
                    y: self.start.y - 1,
                });
            }
        }

        if self.start.x > 0 {
            neighbors.push(Coord {
                x: self.start.x - 1,
                y: self.start.y,
            });
        }

        if self.end.x < M - 1 {
            neighbors.push(Coord {
                x: self.end.x + 1,
                y: self.start.y,
            });
        }

        if self.start.y < N - 1 {
            for x in start_x..end_x + 1 {
                neighbors.push(Coord {
                    x,
                    y: self.start.y + 1,
                });
            }
        }

        return neighbors;
    }
}

#[derive(Debug)]
struct Schematic {
    scheme: [char; M * N],
    symbols: Vec<Coord>,
    parts: Vec<SchemaItem>,
}

fn to_coord(index: usize) -> Coord {
    return Coord {
        y: (index / M),
        x: (index % M),
    };
}

impl Schematic {
    pub fn new(map: &[char; M * N]) -> Self {
        let mut parts = Vec::new();
        let symbols: Vec<_> = map
            .iter()
            .enumerate()
            .filter(|(_, c)| !(c.is_ascii_digit() || *c == &'.'))
            .map(|(idx, _)| to_coord(idx))
            .collect();

        for row in 0..N {
            let mut found_item = "".to_string();
            // iterate over all columns, finding digits
            for col in 0..M {
                let cur_char = map[M * row + col];
                if cur_char.is_digit(10) {
                    found_item += &map[M * row + col].to_string();
                }

                // if we reached the end of a part
                if !found_item.is_empty() && (col == M - 1 || !cur_char.is_digit(10)) {
                    // we just reached the end of an item

                    let end_x = match cur_char.is_digit(10) {
                        true => col,
                        false => col - 1,
                    };
                    let item = SchemaItem {
                        start: Coord {
                            x: end_x + 1 - found_item.len(),
                            y: row,
                        },
                        end: Coord { x: end_x, y: row },
                        value: found_item.parse().unwrap(),
                    };

                    // check if the item is a part
                    let has_symbol_neighbor = item
                        .get_neighbor_coords()
                        .iter()
                        .any(|c| symbols.contains(c));

                    if has_symbol_neighbor {
                        parts.push(item);
                    }
                    found_item = "".to_string();
                }
            }
        }

        Schematic {
            scheme: *map,
            symbols,
            parts,
        }
    }
}

fn part1() {
    let mut all_chars = Vec::new();
    let lines = read_lines("day03/input").unwrap();
    for line in lines {
        all_chars.extend(line.unwrap().chars())
    }

    let s = Schematic::new(all_chars[..].try_into().unwrap());
    // println!("{:?}", s);
    let sum = s.parts.iter().fold(0, |acc, p| acc + p.value as usize);
    println!("{}", sum);
}

fn part2() {
    let mut all_chars = Vec::new();
    let lines = read_lines("day03/input").unwrap();
    for line in lines {
        all_chars.extend(line.unwrap().chars())
    }

    let s = Schematic::new(all_chars[..].try_into().unwrap());

    let mut possible_gears = HashMap::<Coord, Vec<SchemaItem>>::new();

    for p in s.parts {
        // for each part, find any '*' coordinates that it is adjacent to, and add them to the hashmap
        p.get_neighbor_coords()
            .iter()
            .filter(|c| s.symbols.contains(c) && s.scheme[c.y * M + c.x] == '*')
            .for_each(|c| {
                possible_gears
                    .entry(c.clone())
                    .or_insert(Vec::new())
                    .push(p.clone())
            });
    }

    let sum: u32 = possible_gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0].value * v[1].value)
        .sum();
    println!("{}", sum);
}
fn main() {
    part1();
    part2();
}
