use pathfinding::prelude::dijkstra;

#[derive(Debug)]
struct Map {
    heights: Vec<u8>,
    w: i32,
    h: i32,
    start: Pos,
    end: Pos,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Map {
            heights: Vec::<u8>::new(),
            w: input.lines().next().unwrap().len() as i32,
            h: input.lines().count() as i32,
            start: Pos(0, 0),
            end: Pos(0, 0),
        };
        for line in input.lines() {
            for c in line.chars() {
                if c == 'S' {
                    let index = map.heights.len() as i32;
                    map.start = Pos(index % map.w, index / map.w);
                    map.heights.push(0);
                } else if c == 'E' {
                    let index = map.heights.len() as i32;
                    map.end = Pos(index % map.w, index / map.w);
                    map.heights.push(25);
                } else {
                    let mut ord = c.to_string().as_bytes()[0] as u8;
                    ord -= 0x61;
                    map.heights.push(ord);
                }
            }
        }
        return map;
    }
    fn get_index(&self, x: i32, y: i32) -> usize {
        return (x + (y * self.w)) as usize;
    }
    fn is_valid(&self, x: i32, y: i32) -> bool {
        return x >= 0 && x < self.w && y >= 0 && y < self.h;
    }
    fn height_of(&self, x: i32, y: i32) -> u8 {
        return self.heights[self.get_index(x, y)];
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn neighbours(&self, map: &Map) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let height = map.height_of(x, y) + 1;
        let mut choices = Vec::<Pos>::new();
        if map.is_valid(x - 1, y) && map.height_of(x - 1, y) <= height {
            choices.push(Pos(x - 1, y));
        }
        if map.is_valid(x + 1, y) && map.height_of(x + 1, y) <= height {
            choices.push(Pos(x + 1, y));
        }
        if map.is_valid(x, y - 1) && map.height_of(x, y - 1) <= height {
            choices.push(Pos(x, y - 1));
        }
        if map.is_valid(x, y + 1) && map.height_of(x, y + 1) <= height {
            choices.push(Pos(x, y + 1));
        }
        choices.into_iter().map(|p| (p, 1)).collect()
    }

    fn neighbours_p2(&self, map: &Map) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let height = map.height_of(x, y) - 1;
        let mut choices = Vec::<Pos>::new();
        if map.is_valid(x - 1, y) && map.height_of(x - 1, y) >= height {
            choices.push(Pos(x - 1, y));
        }
        if map.is_valid(x + 1, y) && map.height_of(x + 1, y) >= height {
            choices.push(Pos(x + 1, y));
        }
        if map.is_valid(x, y - 1) && map.height_of(x, y - 1) >= height {
            choices.push(Pos(x, y - 1));
        }
        if map.is_valid(x, y + 1) && map.height_of(x, y + 1) >= height {
            choices.push(Pos(x, y + 1));
        }
        choices.into_iter().map(|p| (p, 1)).collect()
    }
}

fn part1(input: &str) {
    let map = Map::new(input);
    let mut point = map.start.clone();
    let mut target = map.end.clone();
    let result = dijkstra(&point, |p: &Pos| p.neighbours(&map), |p| *p == target);
    println!("Part 1: {}", result.unwrap().1);
}

fn part2(input: &str) {
    let map = Map::new(input);
    let mut point = map.end.clone();
    let result = dijkstra(
        &point,
        |p: &Pos| p.neighbours_p2(&map),
        |p| map.height_of(p.0, p.1) == 0,
    );
    println!("Part 2: {}", result.unwrap().1);
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}
