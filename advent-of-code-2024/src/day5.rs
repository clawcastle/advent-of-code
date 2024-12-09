use std::collections::{HashMap, HashSet};

pub fn part1() {
    let input = parse_input();

    let result: i32 = input.updates.iter().filter(|update| input.update_rules.is_valid(*update)).map(|update| update.middle_number()).sum();

    println!("Day 5 part 1 solution: {}", result);
}

fn parse_input() -> Input {
    let file_content = std::fs::read_to_string("./input/day5.txt").expect("Couldn't find input file.");

    let (update_rules_part, updates_part) = file_content.split_once("\n\n").expect("Failed to split input in rules and updates.");

    let update_rules: Vec<UpdateRule> = update_rules_part.lines().map(|line| {
        let (n1, n2): (i32, i32)  = line.split_once('|').map(|(s1, s2)| (s1.parse().expect("Failed to parse n1"), s2.parse().expect("Failed to parse n2"))).unwrap();

        UpdateRule(n1, n2)
    }).collect();

    let updates = updates_part.lines()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().expect("Failed to parse update number")))
        .map(|nums| Update(nums.collect()))
        .collect();

    Input {
        updates,
        update_rules: UpdateRules::new(update_rules)
    }
}

#[derive(Debug, Clone)]
struct Input {
    update_rules: UpdateRules,
    updates: Vec<Update>
}

#[derive(Debug, Clone, Copy)]
struct UpdateRule(i32, i32);

#[derive(Debug, Clone)]
struct Update(Vec<i32>);

impl Update {
    fn middle_number(&self) -> i32 {
        let mid_index = self.0.len() / 2;

        let middle_numer = self.0[mid_index];

        println!("middle number for {:?} is {}", &self.0, middle_numer);
        middle_numer
    }
}

#[derive(Debug, Clone)]
struct UpdateRules {
    nums_not_allowed_after: HashMap<i32, HashSet<i32>>
}

impl UpdateRules {
    fn new(rules: Vec<UpdateRule>) -> Self {
        let mut nums_not_allowed_after: HashMap<i32, HashSet<i32>> = HashMap::new();

        rules.iter().for_each(|rule| {
            nums_not_allowed_after.entry(rule.0).or_insert(HashSet::new()).insert(rule.1);
        });

        Self { nums_not_allowed_after }
    }

    fn is_valid(&self, update: &Update) -> bool {
        for (i, n) in update.0.iter().enumerate() {
            let not_allowed = self.nums_not_allowed_after.get(n);

            let nums_to_check = &update.0[..i];

            for nn in nums_to_check {
                if not_allowed.map(|not_allowed| not_allowed.contains(nn)).unwrap_or(false) {
                    // println!("update {:?} is invalid. not allowed: {:?}. Checking nums: {:?}", &update.0, &not_allowed, &nums_to_check);
                    
                    return false;
                }
            }
        }

        true
    }
}