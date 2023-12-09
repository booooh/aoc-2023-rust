use common::read_lines;

fn main() {
    let lines = read_lines("day01/input").unwrap();
    let mut sum = 0;
    for line in lines {
        let digits : Vec<_> = line.unwrap().chars().filter_map(|c| c.to_digit(10)).collect();
        println!("{:?}", digits);
        sum += digits[0] * 10;
        sum += digits[..].last().unwrap();
        println!("{}", sum);
    }
    // for each line in lines, split digits
}
