use regex::Regex;

//https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

struct Directory {
    name: String,
    parent: usize,
    first_child: usize,
    last_child: usize,
    files_size_total: u32,
}

fn create_directories(input: &str) -> Vec<Directory> {
    let re_cd = Regex::new(r"^\$ cd (.*)$").unwrap();
    let re_dir = Regex::new(r"^dir (.*)$").unwrap();
    let re_file = Regex::new(r"^(\d+) .*$").unwrap();
    let mut directories = Vec::<Directory>::new();
    directories.push(Directory {
        name: String::from("/"),
        parent: 0,
        first_child: 0,
        last_child: 0,
        files_size_total: 0,
    });
    let mut index = 0;
    for line in input.lines() {
        if re_cd.is_match(line) {
            let dir = &re_cd.captures(line).unwrap()[1];
            if dir == ".." {
                index = directories[index].parent;
            } else if dir == "/" {
                index = 0;
            } else {
                let mut ptr = directories[index].first_child;
                assert!(ptr != 0);
                while ptr <= directories[index].last_child {
                    if directories[ptr].name == dir.to_string() {
                        index = ptr;
                        break;
                    }
                    ptr += 1;
                }
            }
        }
        if re_dir.is_match(line) {
            let dir = String::from(&re_dir.captures(line).unwrap()[1]);
            let current_index = directories.len();
            if directories[index].first_child == 0 {
                directories[index].first_child = current_index;
            }
            directories[index].last_child = current_index;
            directories.push(Directory {
                name: dir,
                parent: index,
                first_child: 0,
                last_child: 0,
                files_size_total: 0,
            });
        }
        if re_file.is_match(line) {
            let size = &re_file.captures(line).unwrap()[1].parse::<u32>().unwrap();
            let mut ptr = index;
            while ptr > 0 {
                directories[ptr].files_size_total += size;
                ptr = directories[ptr].parent;
            }
            directories[0].files_size_total += size;
        }
    }
    return directories;
}

fn part1(input: &str) -> u32 {
    let directories = create_directories(input);
    let mut result = 0;
    for d in directories {
        if d.files_size_total <= 100000 {
            result += d.files_size_total;
        }
    }

    return result;
}

fn part2(input: &str) -> u32 {
    let directories = create_directories(input);
    let used_space = directories[0].files_size_total;
    let free_space = 70000000 - used_space;
    let extra_needed = 30000000 - free_space;
    let mut result = 70000000;
    for d in directories {
        if d.files_size_total >= extra_needed && d.files_size_total < result {
            result = d.files_size_total;
        }
    }

    return result;
}

fn main() {
    static input: &str = include_str!("../input.txt");
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
