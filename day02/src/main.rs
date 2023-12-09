use common::read_lines;
use regex::Regex;
use std::cmp;

#[derive(Debug, Clone)]
struct GameResults {
    red: u32,
    green: u32,
    blue: u32,
}

struct RoundResults {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_subset(subset: &str) -> GameResults {
    let mut res = GameResults { red: 0, blue:0, green: 0 };
    let cubes_picked = subset.split(",");
    for p in cubes_picked {
        let val = p.trim().split(" ").next().unwrap().parse::<u32>();
        if val.is_err() {
            println!("could not parse {}", p.trim());
            continue;
        }
        let count = val.unwrap();
        if p.contains("blue") {
            res.blue = count;
        } else if p.contains("green") {
            res.green = count;
        } else if p.contains("red") {
            res.red = count;
        }
    }
    return res;

}

fn parse_game_line(line: &str) -> (u32, Vec<GameResults>)
{
    let mut res = Vec::new();
    let re = Regex::new(r"Game (?<id>[0-9]*): (?<rest>([^;]*;)*)(?<last>[^;]*)").unwrap();
    let mut id : u32 = 0;
    for cap in re.captures_iter(line) {
         id = (&cap["id"]).trim().parse::<u32>().unwrap();
         let mut all_picks = vec![&cap["last"]];
         all_picks.extend((&cap["rest"]).split(";"));
         res = all_picks.iter().filter(|&x| !x.trim().is_empty() ).map(|subset| parse_subset(subset)).collect();
    }
    return (id, res);
}

fn get_max_cubes(game: Vec<GameResults>) -> GameResults{
    let mut res = game[0].clone();
    for pick in game {
        res.red = cmp::max(res.red, pick.red);
        res.green = cmp::max(res.green, pick.green);
        res.blue = cmp::max(res.blue, pick.blue);
    }

    return res;
}

fn part1() {
    let mut sum : u32 = 0;
    let lines = read_lines("day02/input").unwrap();
    for line in lines {
        let foo = parse_game_line(&line.unwrap());
        let res = (foo.0, get_max_cubes(foo.1));
        if res.1.red <=12 && res.1.green <= 13 && res.1.blue <=14 {
            println!("{:?}", res);
            sum += res.0;
        }
    }
    println!("{:?}", sum);
}
fn main() {
    part1();
}
