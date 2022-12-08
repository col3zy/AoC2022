fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    let mut index = 4;
    while true {
        let a = chars.next().unwrap();
        let mut current = chars.clone();
        let b = current.next().unwrap();
        let c = current.next().unwrap();
        let d = current.next().unwrap();
        if a == b || a == c || a == d || b == c || b == d || c == d {
            index += 1;
        } else {
            return index;
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let mut chars = input.chars();
    let mut index = 14;
    while true {
        let mut current: Vec<char> = chars.clone().take(14).collect();
        chars.next();
        current.sort();
        current.dedup();
        if current.len() == 14 {
            return index;
        }
        index += 1;
    }
    0
}

fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../test.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests1() {
        assert_eq!(part1(include_str!("../test.txt")), 7);
        assert_eq!(part2(include_str!("../test.txt")), 19);
    }
}
