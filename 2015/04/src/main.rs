use hex_literal::hex;

fn part1(input: &str) -> usize {
    // create a Md5 hasher instance
    let mut index = 0;
    while true {
        let line = format!("{}{}", input, index);
        index += 1;
        let digest = md5::compute(line.clone());
    }
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input = "bgvyzdsv";
    //let input = include_str!("../test.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests1() {
        assert_eq!(part1("abcdef"), 609043);
    }
}
