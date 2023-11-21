use std::time::Instant;

fn part1(input: &str) -> i32 {
    let elves = input.split("\r\n\r\n");
    let calories = elves.map(|x| {
        x.lines()
            .map(|l| -> i32 { return l.trim().parse().unwrap() })
            .sum()
    });
    return calories.max().unwrap();
}

fn part2(input: &str) -> i32 {
    let calories = input.split("\r\n\r\n").map(|x| {
        x.lines()
            .map(|l| -> i32 { return l.trim().parse().unwrap() })
            .sum()
    });
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for x in calories {
        if x > c {
            c = x;
        }
        if c > b {
            let t = c;
            c = b;
            b = t;
        }
        if b > a {
            let t = a;
            a = b;
            b = t;
        }
    }
    return a + b + c;
}

fn izzy_p2(input: &str) -> i32 {
    let mut elf_sum = 0;
    let mut elf_sums: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            elf_sums.push(elf_sum);
            elf_sum = 0;
        } else {
            elf_sum += line.parse::<i32>().unwrap();
        }
    }
    elf_sums.push(elf_sum);
    elf_sums.sort_by(|a, b| b.cmp(a));
    elf_sums[0..3].iter().sum()
}

fn fast_p2(input: &str) -> i32 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut t = 0;
    let mut x = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            if x > c {
                c = x;
            }
            if c > b {
                t = c;
                c = b;
                b = t;
            }
            if b > a {
                t = a;
                a = b;
                b = t;
            }
            x = 0;
        } else {
            x += line.parse::<i32>().unwrap();
        }
    }
    if x > c {
        c = x;
    }
    return a + b + c;
}

fn main() {
    let input = include_str!("../input.txt");

    let mut start: Instant;
    let mut execution_time = 0;

    let mut average_execution_time: u128 = 0;
    for _c in 0..20 {
        let start = Instant::now();
        part2(input);
        average_execution_time += start.elapsed().as_micros();
    }
    average_execution_time = average_execution_time / 20;
    println!(
        "The average execution time is {} microseconds",
        average_execution_time
    );

    let mut average_execution_time: u128 = 0;
    for _c in 0..20 {
        let start = Instant::now();
        izzy_p2(input);
        average_execution_time += start.elapsed().as_micros();
    }
    average_execution_time = average_execution_time / 20;
    println!(
        "The average execution time is {} microseconds",
        average_execution_time
    );

    let mut average_execution_time: u128 = 0;
    for _c in 0..20 {
        let start = Instant::now();
        fast_p2(input);
        average_execution_time += start.elapsed().as_micros();
    }
    average_execution_time = average_execution_time / 20;
    println!(
        "The average execution time is {} microseconds",
        average_execution_time
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }
}
