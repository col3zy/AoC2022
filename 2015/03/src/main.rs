use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut houses: HashMap<String, i32> = HashMap::new();
    houses.insert("0x0".to_string(), 1);
    for c in input.chars() {
        if c == '^' {
            y += 1;
        }
        if c == '>' {
            x += 1;
        }
        if c == 'v' {
            y -= 1;
        }
        if c == '<' {
            x -= 1;
        }
        let address = format!("{}x{}", x, y);
        houses
            .entry(address)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    return houses.len();
}

fn part2(input: &str) -> usize {
    let mut coord = [(0, 0), (0, 0)];
    let mut turn = 0;
    let mut houses: HashMap<String, i32> = HashMap::new();
    houses.insert("0x0".to_string(), 1);
    for c in input.chars() {
        if c == '^' {
            coord[turn].1 += 1;
        }
        if c == '>' {
            coord[turn].0 += 1;
        }
        if c == 'v' {
            coord[turn].1 -= 1;
        }
        if c == '<' {
            coord[turn].0 -= 1;
        }
        let address = format!("{}x{}", coord[turn].0, coord[turn].1);
        houses
            .entry(address)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        turn += 1;
        turn = turn % 2;
    }

    return houses.len();
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
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
        //assert_eq!(part2("2x3x4"), 34);
    }
}
