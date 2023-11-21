use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let nums = input.lines().map(|v| {
        assert!(re.is_match(v));
        let c = re.captures(v).unwrap();
        let a = &c[1].parse::<u8>().unwrap();
        let b = &c[2].parse::<u8>().unwrap();
        let x = &c[3].parse::<u8>().unwrap();
        let y = &c[4].parse::<u8>().unwrap();

        if (x <= a && y >= b) || (a <= x && b >= y) {
            return 1;
        } else {
            return 0;
        }
    });
    return nums.sum();
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let nums = input.lines().map(|v| {
        assert!(re.is_match(v));
        let c = re.captures(v).unwrap();
        let a = &c[1].parse::<u8>().unwrap();
        let b = &c[2].parse::<u8>().unwrap();
        let x = &c[3].parse::<u8>().unwrap();
        let y = &c[4].parse::<u8>().unwrap();

        if a >= x && a <= y {
            return 1;
        }
        if b >= x && b <= y {
            return 1;
        }
        if x >= a && x <= b {
            return 1;
        }
        if y >= a && y <= b {
            return 1;
        }
        return 0;
    });
    return nums.sum();
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests1() {
        assert_eq!(part1(include_str!("../test.txt")), 2);
    }
}
