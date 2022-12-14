use std::cmp::Ordering;

use json::JsonValue;

#[derive(PartialEq)]
enum Result {
    FALSE,
    TRUE,
    NEXT,
}

fn compare(lhs: &JsonValue, rhs: &JsonValue, depth: u8) -> Result {
    for _ in 0..depth {
        print!("  ");
    }
    println!("- Compare {} vs {}", lhs, rhs);
    if lhs.is_number() && rhs.is_number() {
        if lhs.as_i32().unwrap() > rhs.as_i32().unwrap() {
            for _ in 0..depth {
                print!("  ");
            }
            println!("- Right side is smaller, so inputs are not in the right order");
            return Result::FALSE;
        }
        if lhs.as_i32().unwrap() < rhs.as_i32().unwrap() {
            for _ in 0..depth {
                print!("  ");
            }
            println!("- Left side is smaller, so inputs are in the right order");
            return Result::TRUE;
        }
    }
    if lhs.is_array() && rhs.is_array() {
        let limit = rhs.members().count();
        for i in 0..lhs.members().count() {
            if i == limit {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("  - Right side ran out of items, so inputs are not in the right order");
                return Result::FALSE;
            }
            let res = compare(&lhs[i], &rhs[i], depth + 1);
            if res != Result::NEXT {
                return res;
            }
        }
        if limit > lhs.members().count() {
            return Result::TRUE;
        }
    }
    if lhs.is_array() && rhs.is_number() {
        let limit = 1;
        for i in 0..lhs.members().count() {
            if i == limit {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("  - Right side ran out of items, so inputs are not in the right order");
                return Result::FALSE;
            }
            let res = compare(&lhs[i], &rhs, depth + 1);
            if res != Result::NEXT {
                return res;
            }
        }
        if limit > lhs.members().count() {
            return Result::TRUE;
        }
    }
    if lhs.is_number() && rhs.is_array() {
        if rhs.members().count() < 1 {
            return Result::FALSE;
        }
        let res = compare(&lhs, &rhs[0], depth + 1);
        if res != Result::NEXT {
            return res;
        }
        if rhs.members().count() > 1 {
            return Result::TRUE;
        }
    }
    Result::NEXT
}

fn check_order(block: &str) -> bool {
    let mut lines = block.lines();
    let lhs = json::parse(lines.next().unwrap()).unwrap();
    let rhs = json::parse(lines.next().unwrap()).unwrap();
    return compare(&lhs, &rhs, 0) != Result::FALSE;
}

fn part1(input: &str) {
    let mut count = 1;
    let mut sum = 0;
    for block in input.split("\r\n\r\n") {
        println!("\n== Pair {} ==", count);
        if check_order(block) {
            println!("TRUE {}", count);
            sum += count;
        } else {
            println!("FALSE {}", count);
        }
        count += 1;
    }
    println!("Part 1: {}", sum);
}

fn sorter(lhs: &JsonValue, rhs: &JsonValue) -> Ordering {
    if compare(&lhs, &rhs, 0) != Result::FALSE {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn part2(input: &str) {
    let mut data = Vec::<JsonValue>::new();
    for block in input.split("\r\n\r\n") {
        let mut lines = block.lines();
        data.push(json::parse(lines.next().unwrap()).unwrap());
        data.push(json::parse(lines.next().unwrap()).unwrap());
    }
    let n1 = json::parse("[[2]]").unwrap();
    let n2 = json::parse("[[6]]").unwrap();
    data.push(n1.clone());
    data.push(n2.clone());
    data.sort_by(sorter);
    let mut sum = 1;
    for (i, d) in data.iter().enumerate() {
        if d.eq(&n1) || d.eq(&n2) {
            sum *= i + 1;
            println!("{:3}: {}", i, d);
        }
    }
    println!("Part2: {}", sum);
}

fn main() {
    let input = include_str!("../input.txt");
    part2(input);
}
