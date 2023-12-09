use common::read_lines;

fn part1() {
    let lines = read_lines("day01/input").unwrap();
    let mut sum = 0;
    // for each line in lines, split digits
    for line in lines {
        let digits : Vec<_> = line.unwrap().chars().filter_map(|c| c.to_digit(10)).collect();
        sum += digits[0] * 10;
        sum += digits[..].last().unwrap();
    }
    println!("{}", sum);
}

fn is_digit_or_word(substr : &str) -> Option<u32> {
    // given a substring, returns whether it starts with a digit, or a word that speels out a digit
    let digits = ["0", "zero", "1", "one", "2", "two", "3", "three", "4", "four",  "5", "five", "6", "six", "7", "seven","8", "eight", "9", "nine"];
    for (idx, d) in digits.iter().enumerate(){
        if substr.starts_with(d) {
            return Some((idx /2) as u32)
        }
    }
    return None
}

fn part2() {
    let lines = read_lines("day01/input").unwrap();
    let mut sum = 0;
    for line in lines {
        let l = line.unwrap();
        let digits: Vec<_> = l.char_indices().map(|(idx,_)| &l[idx..] ).filter_map(|w| is_digit_or_word(w)).collect();
        sum += digits[0] * 10;
        sum += digits[..].last().unwrap();
    }
    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
