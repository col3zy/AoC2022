fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();
        score += 1;
        if me == 'Y' {
            score += 1;
        }
        if me == 'Z' {
            score += 2;
        }

        if (opp == 'A' && me == 'Y') || (opp == 'B' && me == 'Z') || (opp == 'C' && me == 'X') {
            score += 6;
        }
        if (opp == 'A' && me == 'X') || (opp == 'B' && me == 'Y') || (opp == 'C' && me == 'Z') {
            score += 3;
        }
    }
    return score;
}

fn part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let opp = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();
        let mut me = 'A';

        if opp == 'A' {
            if outcome == 'X' {
                me = 'C';
            }
            if outcome == 'Y' {
                me = 'A';
            }
            if outcome == 'Z' {
                me = 'B';
            }
        }
        if opp == 'B' {
            if outcome == 'X' {
                me = 'A';
            }
            if outcome == 'Y' {
                me = 'B';
            }
            if outcome == 'Z' {
                me = 'C';
            }
        }
        if opp == 'C' {
            if outcome == 'X' {
                me = 'B';
            }
            if outcome == 'Y' {
                me = 'C';
            }
            if outcome == 'Z' {
                me = 'A';
            }
        }

        if outcome == 'Y' {
            score += 3;
        }
        if outcome == 'Z' {
            score += 6;
        }

        if me == 'A' {
            score += 1;
        }
        if me == 'B' {
            score += 2;
        }
        if me == 'C' {
            score += 3;
        }
    }
    return score;
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
        assert_eq!(part1("A Y\nB X\nC Z"), 15);
        assert_eq!(part2("A Y\nB X\nC Z"), 12);
    }
}
