fn bag(line: &str) -> u32 {
    let line_size = line.len();
    let half_point = line_size / 2;
    let lhs = &line[0..half_point];
    let rhs = &line[half_point..line_size];
    for c in lhs.chars() {
        if rhs.find(c) != None {
            let mut ord = c.to_string().as_bytes()[0] as u32;
            if ord > 0x60 {
                ord -= 0x60;
            }
            if ord > 0x40 {
                ord -= 0x40;
                ord += 26;
            }
            return ord;
        }
    }
    return 0;
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| -> u32 { bag(line) }).sum()
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u32 = 0;
    let mut lines = input.lines();
    while true {
        let test = lines.next();
        if test == None {
            break;
        }
        let a = test.unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();
        for v in a.chars() {
            if b.find(v) != None && c.find(v) != None {
                let mut ord = v.to_string().as_bytes()[0] as u32;
                if ord > 0x60 {
                    ord -= 0x60;
                }
                if ord > 0x40 {
                    ord -= 0x40;
                    ord += 26;
                }
                total += ord;
                break;
            }
        }
    }
    return total;
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
        assert_eq!(bag("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(bag("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(bag("PmmdzqPrVvPwwTWBwg"), 42);
        assert_eq!(bag("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
        assert_eq!(bag("ttgJtRGJQctTZtZT"), 20);
        assert_eq!(bag("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
        assert_eq!(part1("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"), 157);
        assert_eq!(
            part2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg"),
            18
        );
        assert_eq!(part2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"), 70);
    }
}
