use regex::Regex;

fn need_to_move(hx: i32, hy: i32, tx: i32, ty: i32) -> bool {
    let x = (hx - tx).abs();
    let y = (hy - ty).abs();
    return x > 1 || y > 1;
}

fn part2(input: &str) -> usize {
    //get grid dimensions
    let re = Regex::new(r"^(.) (\d+)$").unwrap();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut maxX: i32 = 0;
    let mut minX: i32 = 0;
    let mut maxY: i32 = 0;
    let mut minY: i32 = 0;
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let dir = &captures[1];
        let count = captures[2].parse::<i32>().unwrap();
        if dir == "R" {
            x += count;
        }
        if dir == "L" {
            x -= count;
        }
        if dir == "U" {
            y += count;
        }
        if dir == "D" {
            y -= count;
        }
        if x > maxX {
            maxX = x;
        }
        if x < minX {
            minX = x;
        }
        if y > maxY {
            maxY = y;
        }
        if y < minY {
            minY = y;
        }
    }

    let w: i32 = 2 + maxX - minX;
    let h: i32 = 2 + maxY - minY;
    let mut hx: i32 = 1 - minX;
    let mut hy: i32 = 1 - minY;
    println!("grid {},{} - start {},{}", w, h, hx, hy);
    let mut tx = [hx, hx, hx, hx, hx, hx, hx, hx, hx, hx];
    let mut ty = [hy, hy, hy, hy, hy, hy, hy, hy, hy, hy];
    let mut visited = Vec::<i32>::new();
    let mut points: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    for _ in 0..10 {
        points.push(Vec::<i32>::new())
    }
    visited.push(hx + (hy * w));
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let dir = &captures[1];
        let count = captures[2].parse::<i32>().unwrap();
        for c in 0..count {
            if dir == "R" {
                tx[0] += 1;
            }
            if dir == "L" {
                tx[0] -= 1;
            }
            if dir == "U" {
                ty[0] += 1;
            }
            if dir == "D" {
                ty[0] -= 1;
            }
            for i in 1..10 {
                if need_to_move(tx[i - 1], ty[i - 1], tx[i], ty[i]) {
                    if (ty[i - 1] - ty[i]).abs() > 0 {
                        ty[i] += (ty[i - 1] - ty[i]) / (ty[i - 1] - ty[i]).abs();
                    }
                    if (tx[i - 1] - tx[i]).abs() > 0 {
                        tx[i] += (tx[i - 1] - tx[i]) / (tx[i - 1] - tx[i]).abs();
                    }
                    if i == 9 {
                        visited.push(tx[i] + (ty[i] * w));
                    }
                }
            }
        }
    }
    visited.sort();
    visited.dedup();
    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x + ((h - y) * w))) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    return visited.len();
}

fn main() {
    static input: &str = include_str!("../input.txt");
    println!("{}", part2(input));
}
