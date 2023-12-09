use common::read_lines;

fn part1() {
    let lines = read_lines("day01/input_part1").unwrap();
    let mut sum = 0;
    // for each line in lines, split digits
    for line in lines {
        let digits : Vec<_> = line.unwrap().chars().filter_map(|c| c.to_digit(10)).collect();
        sum += digits[0] * 10;
        sum += digits[..].last().unwrap();
    }
    println!("{}", sum);
}

fn main() {
    part1();
}
