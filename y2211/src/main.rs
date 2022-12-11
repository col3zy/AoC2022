use regex::Regex;
use std::collections::VecDeque;

static PRINT: bool = false;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation_add: bool,
    val_const: u128,
    val_old: bool,
    test_divisor: u128,
    target_true: usize,
    target_false: usize,
    business: usize,
}

impl Monkey {
    fn new(input: &str) -> Self {
        let re_get_number = Regex::new(r"(\d+)").unwrap();
        let re_operation = Regex::new(r"new = old (.) (.*)$").unwrap();
        let mut lines = input.lines();
        lines.next(); // skip title

        let mut monkey = Monkey {
            items: VecDeque::<u128>::new(),
            operation_add: false,
            val_const: 0,
            val_old: false,
            test_divisor: 0,
            target_true: 0,
            target_false: 0,
            business: 0,
        };

        // Starting items
        for cap in re_get_number.captures_iter(lines.next().unwrap()) {
            monkey.items.push_back(cap[1].parse::<u128>().unwrap());
        }

        // Operation
        let cap = re_operation.captures(lines.next().unwrap()).unwrap();
        if &cap[1] == "+" {
            monkey.operation_add = true;
        }
        if &cap[2] == "old" {
            monkey.val_old = true;
        } else {
            monkey.val_const = cap[2].parse::<u128>().unwrap()
        }

        // Test
        monkey.test_divisor = re_get_number.captures(lines.next().unwrap()).unwrap()[1]
            .parse::<u128>()
            .unwrap();
        monkey.target_true = re_get_number.captures(lines.next().unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        monkey.target_false = re_get_number.captures(lines.next().unwrap()).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        return monkey;
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
    round: usize,
    turn: usize,
    turn_divisor: u128,
}

impl Troop {
    fn new(input: &str) -> Self {
        let mut monkeys = Vec::<Monkey>::new();
        for substr in input.split("\r\n\r\n") {
            monkeys.push(Monkey::new(substr));
        }
        return Troop {
            monkeys: monkeys,
            round: 0,
            turn: 0,
            turn_divisor: 3,
        };
    }

    fn simulate_monkey(&mut self, index: usize) {
        let count = self.monkeys[index].items.len();
        self.monkeys[index].business += count;
        if PRINT {
            println!("Monkey {}:", index);
        }
        for _ in 0..count {
            let mut value: u128 = self.monkeys[index].items.pop_front().unwrap();
            if PRINT {
                println!("  Monkey inspects an item with a worry level of {}.", value);
            }

            let op_val = if self.monkeys[index].val_old {
                value
            } else {
                self.monkeys[index].val_const
            };
            let operation_text = if self.monkeys[index].operation_add {
                value += op_val;
                "increases"
            } else {
                value *= op_val;
                "is multiplied"
            };

            if PRINT {
                println!(
                    "    Worry level {} by {} to {}.",
                    operation_text, op_val, value
                );
            }

            if self.turn_divisor > 1 {
                value /= self.turn_divisor;
            }

            value %= 223092870;
            if PRINT {
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    value
                );
            }

            let result = value % self.monkeys[index].test_divisor == 0;
            if PRINT {
                let not_text = if result { "" } else { "not " };
                println!(
                    "    Current worry level is {}divisible by {}.",
                    not_text, self.monkeys[index].test_divisor
                );
            }

            let target = if result {
                self.monkeys[index].target_true
            } else {
                self.monkeys[index].target_false
            };
            if PRINT {
                println!(
                    "    Item with worry level {} is thrown to monkey {}.",
                    value, target
                );
            }
            self.monkeys[target].items.push_back(value);
        }
        if PRINT {
            println!("");
        }
    }

    fn do_turn(&mut self) {
        self.simulate_monkey(self.turn);
        self.turn += 1;
        self.turn %= self.monkeys.len();
    }

    fn do_round(&mut self) {
        self.do_turn();
        while self.turn > 0 {
            self.do_turn();
        }
        self.round += 1;
    }

    fn part1(&mut self) {
        for _ in 0..20 {
            self.do_round();
        }
        let mut businesses = Vec::<usize>::new();
        for monkey in self.monkeys.iter() {
            businesses.push(monkey.business);
        }
        businesses.sort_by(|a, b| b.cmp(a));
        println!("Part1: {}", businesses[0] * businesses[1]);
    }

    fn part2(&mut self) {
        for _ in 0..10000 {
            self.do_round();
        }
        let mut businesses = Vec::<usize>::new();
        for monkey in self.monkeys.iter() {
            businesses.push(monkey.business);
        }
        businesses.sort_by(|a, b| b.cmp(a));
        println!("Part2: {}", businesses[0] * businesses[1]);
    }
}

fn part1(input: &str) {
    let mut troop = Troop::new(input);
    troop.part1();
}

fn part2(input: &str) {
    let mut troop = Troop::new(input);
    troop.turn_divisor = 1;
    troop.part2();
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}
