fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| -> i32 {
            let nums = line.split("x").map(|v| v.parse::<i32>().unwrap());
            let mut arr = nums.collect::<Vec<i32>>();
            arr.sort();
            return (3 * arr[0] * arr[1]) + 2 * (arr[0] * arr[2] + arr[1] * arr[2]);
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| -> i32 {
            let nums = line.split("x").map(|v| v.parse::<i32>().unwrap());
            let mut arr = nums.collect::<Vec<i32>>();
            arr.sort();

            return (2 * (arr[0] + arr[1])) + arr[0] * arr[1] * arr[2];
        })
        .sum()
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
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }
}
