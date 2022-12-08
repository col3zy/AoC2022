struct Tree {
    height: usize,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

fn parse(input: &str) -> Vec<Tree> {
    let mut forest: Vec<Tree> = Vec::<Tree>::new();
    // Edge node
    forest.push(Tree {
        height: 0,
        up: 0,
        down: 0,
        left: 0,
        right: 0,
    });
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        for c in line.chars() {
            let mut left = 0;
            let mut up = 0;
            let mut right = 0;
            let mut down = 0;
            if x > 0 {
                left = (y * w) + x - 1;
                left += 1;
            }
            if x < w - 1 {
                right = (y * w) + x + 1;
                right += 1;
            }
            if y > 0 {
                up = ((y - 1) * w) + x;
                up += 1;
            }
            if y < h - 1 {
                down = ((y + 1) * w) + x;
                down += 1;
            }
            let t = Tree {
                height: c.to_string().parse::<usize>().unwrap(),
                up: up,
                down: down,
                left: left,
                right: right,
            };
            forest.push(t);
            x += 1;
            if x == w {
                x = 0;
                y += 1;
            }
        }
    }
    return forest;
}

fn part1(input: &str) -> u32 {
    let forest = parse(input);
    let mut visible_count = 0;
    for tree in &forest {
        if tree.up == 0 || tree.down == 0 || tree.left == 0 || tree.right == 0 {
            visible_count += 1;
            continue;
        }
        let mut index = tree.up;
        let mut blocked = false;
        while index > 0 {
            if forest[index].height >= tree.height {
                blocked = true;
                break;
            }
            index = forest[index].up;
        }
        if !blocked {
            visible_count += 1;
            continue;
        }
        index = tree.down;
        blocked = false;
        while index > 0 {
            if forest[index].height >= tree.height {
                blocked = true;
                break;
            }
            index = forest[index].down;
        }
        if !blocked {
            visible_count += 1;
            continue;
        }
        index = tree.left;
        blocked = false;
        while index > 0 {
            if forest[index].height >= tree.height {
                blocked = true;
                break;
            }
            index = forest[index].left;
        }
        if !blocked {
            visible_count += 1;
            continue;
        }
        index = tree.right;
        blocked = false;
        while index > 0 {
            if forest[index].height >= tree.height {
                blocked = true;
                break;
            }
            index = forest[index].right;
        }
        if !blocked {
            visible_count += 1;
            continue;
        }
    }
    return visible_count - 1;
}

fn part2(input: &str) -> u32 {
    let forest = parse(input);
    let mut best_score = 0;
    for tree in &forest {
        let mut score = 0;
        let mut index = tree.up;
        while index > 0 {
            score += 1;
            if forest[index].height >= tree.height {
                break;
            }
            index = forest[index].up;
        }
        index = tree.down;
        let mut count = 0;
        while index > 0 {
            count += 1;
            if forest[index].height >= tree.height {
                break;
            }
            index = forest[index].down;
        }
        score *= count;
        count = 0;
        index = tree.left;
        while index > 0 {
            count += 1;
            if forest[index].height >= tree.height {
                break;
            }
            index = forest[index].left;
        }
        score *= count;
        count = 0;
        index = tree.right;
        while index > 0 {
            count += 1;
            if forest[index].height >= tree.height {
                break;
            }
            index = forest[index].right;
        }
        score *= count;
        if score > best_score {
            best_score = score
        }
    }
    return best_score;
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
        assert_eq!(part1(include_str!("../test.txt")), 21);
        assert_eq!(part2(include_str!("../test.txt")), 8);
    }
}
