use regex::Regex;

fn parse_initial(input: &str) -> Vec<Vec<char>> {
    let table = input.split("\r\n\r\n").nth(0).unwrap();
    let rows = table.lines().count() - 1;
    let count = (table.lines().nth(0).unwrap().len() + 1) / 4;
    let mut group = Vec::<Vec<char>>::with_capacity(count);
    for i in 0..count {
        group.push(Vec::<char>::with_capacity(rows * count))
    }
    for line in table.lines().take(table.lines().count() - 1) {
        for i in 0..count {
            let offset = (i * 4) + 1;
            let c = line.chars().nth(offset).unwrap();
            if c == ' ' {
                continue;
            }
            group[i].insert(0, c);
        }
    }
    return group;
}

fn part1(input: &str) -> String {
    let mut table = parse_initial(input);
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let instructions = input.split("\r\n\r\n").nth(1).unwrap().lines();
    for inst in instructions {
        assert!(re.is_match(inst));
        let c = re.captures(inst).unwrap();
        let mut num = c[1].parse::<usize>().unwrap();
        let src = &c[2].parse::<usize>().unwrap() - 1;
        let dst = &c[3].parse::<usize>().unwrap() - 1;
        if table[src].len() < num {
            num = table[src].len();
        }
        for i in 0..num {
            let temp = table[src].pop().unwrap();
            table[dst].push(temp);
        }
    }
    let mut result: String = String::new();
    for stack in table {
        result.push(*stack.last().unwrap());
    }
    return result;
}

fn part2(input: &str) -> String {
    let mut table = parse_initial(input);
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let instructions = input.split("\r\n\r\n").nth(1).unwrap().lines();
    for inst in instructions {
        assert!(re.is_match(inst));
        let c = re.captures(inst).unwrap();
        let mut num = c[1].parse::<usize>().unwrap();
        let src = &c[2].parse::<usize>().unwrap() - 1;
        let dst = &c[3].parse::<usize>().unwrap() - 1;
        if table[src].len() < num {
            num = table[src].len();
        }
        let mut temp = Vec::<char>::with_capacity(num);
        for i in 0..num {
            temp.push(table[src].pop().unwrap());
        }

        for i in 0..num {
            table[dst].push(temp.pop().unwrap());
        }
    }
    let mut result: String = String::new();
    for stack in table {
        result.push(*stack.last().unwrap());
    }
    return result;
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
        assert_eq!(part1(include_str!("../test.txt")), "CMZ");
    }
}
