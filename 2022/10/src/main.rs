use regex::Regex;

fn part1(input: &str) -> i32 {
    let re_addx = Regex::new(r"^addx (.*)$").unwrap();
    let mut result = 0;
    let mut clock = 0;
    let mut target = 20;
    let mut x = 1;
    for line in input.lines() {
        clock += 1;
        if re_addx.is_match(line) {
            clock += 1;
        }
        if clock >= target {
            result += x * target;
            target += 40;
        }
        if re_addx.is_match(line) {
            let captures = re_addx.captures(line).unwrap();
            let val = captures[1].parse::<i32>().unwrap();
            x += val;
        }
    }
    if clock >= target {
        result += x * target;
    }
    return result;
}

fn part2(input: &str) {
    let re_noop = Regex::new(r"^noop$").unwrap();
    let re_addx = Regex::new(r"^addx (.*)$").unwrap();
    let mut clock: i32 = 0;
    let mut x: i32 = 1;
    for line in input.lines() {
        if re_noop.is_match(line) {
            if (clock - x).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            clock += 1;
            if clock % 40 == 0 {
                println!("");
                clock = 0;
            }
            continue;
        }
        if re_addx.is_match(line) {
            if (clock - x).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            clock += 1;
            if clock % 40 == 0 {
                println!("");
                clock = 0;
            }
            if (clock - x).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            clock += 1;
            if clock % 40 == 0 {
                println!("");
                clock = 0;
            }
            let captures = re_addx.captures(line).unwrap();
            let val = captures[1].parse::<i32>().unwrap();
            x += val;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
    part2(input);
}
