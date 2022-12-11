use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");

    let mut monkeys = input
        .split("\n\n")
        .enumerate()
        .filter_map(|id_and_section| Monkey::try_from(id_and_section).ok())
        .collect_vec();

    println!("{:?}", monkeys);

    let round_length = monkeys.len();
    let n_rounds = 10_000;

    for _ in 0..n_rounds {
        for id in 0..round_length {
            if let Some(monkey) = monkeys.iter_mut().find(|m| m.id == id) {
                let updates = &monkey.take_turn();

                for update in updates {
                    if let Some(monkey2) = monkeys.iter_mut().find(|m| m.id == update.throw_to) {
                        monkey2.items.push(update.worry_level);
                    }
                }
            }
        }
    }

    let monkey_business = monkeys
        .iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap();

    println!("{:?}", monkey_business);
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    operator: Operator,
    arguments: OperationArgument,
}

impl Operation {
    fn from_input_line(input_line: &str) -> Result<Self, String> {
        let (right, operator_char, left) = input_line
            .split(' ')
            .rev()
            .next_tuple()
            .ok_or(input_line.to_string())?;

        let arguments = if left == "old" && right == "old" {
            Ok(OperationArgument::OldAndOld)
        } else if let Ok(n) = right.parse::<i64>() {
            Ok(OperationArgument::OldAndOther(n))
        } else {
            Err(input_line.to_string())
        }?;

        let operator = match operator_char.chars().next().expect("Should not be empty") {
            '+' => Ok(Operator::Plus),
            '-' => Ok(Operator::Minus),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            _ => Err(input_line.to_string()),
        }?;

        Ok(Operation {
            operator,
            arguments,
        })
    }

    fn compute(&self, a: i64, b: i64) -> i64 {
        match self.operator {
            Operator::Plus => a + b,
            Operator::Minus => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

#[derive(Clone, Copy, Debug)]
enum OperationArgument {
    OldAndOther(i64),
    OldAndOld,
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divisble_by: i64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn throw_to(&self, worry_level: i64) -> usize {
        if (worry_level % self.divisble_by) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl Operator {
    fn character(&self) -> char {
        match self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
        }
    }
}

const STARTING_ITEMS_KEY: &str = "Starting items:";
const OPERATION_KEY: &str = "Operation:";
const TEST_KEY: &str = "Test:";
const IF_TRUE_KEY: &str = "If true:";
const IF_FALSE_KEY: &str = "If false:";

impl<'a> TryFrom<(usize, &'a str)> for Monkey {
    type Error = &'a str;

    fn try_from((id, value): (usize, &'a str)) -> Result<Self, Self::Error> {
        let lines = value.split('\n').map(|line| line.trim()).collect_vec();

        let starting_items_line = lines
            .iter()
            .find(|line| line.starts_with(STARTING_ITEMS_KEY))
            .ok_or(value)?
            .replace(STARTING_ITEMS_KEY, "")
            .replace(',', "");

        let items = starting_items_line
            .split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect_vec();

        let operation_line = lines
            .iter()
            .find(|line| line.starts_with(OPERATION_KEY))
            .ok_or(value)?;
        let operation = match Operation::from_input_line(operation_line) {
            Ok(op) => Ok(op),
            Err(_) => Err(value),
        }?;

        let test_line = lines
            .iter()
            .find(|line| line.starts_with(TEST_KEY))
            .ok_or(value)?;
        let divisble_by = match test_line.split(' ').last() {
            Some(n) => match n.parse::<i64>() {
                Ok(n) => Ok(n),
                Err(_) => Err(value),
            },
            None => Err(value),
        }?;

        let if_true_line = lines
            .iter()
            .find(|line| line.starts_with(IF_TRUE_KEY))
            .ok_or(value)?;
        let if_true = match if_true_line.split(' ').last() {
            Some(n) => match n.parse::<usize>() {
                Ok(n) => Ok(n),
                Err(_) => Err(value),
            },
            None => Err(value),
        }?;
        let if_false_line = lines
            .iter()
            .find(|line| line.starts_with(IF_FALSE_KEY))
            .ok_or(value)?;
        let if_false = match if_false_line.split(' ').last() {
            Some(n) => match n.parse::<usize>() {
                Ok(n) => Ok(n),
                Err(_) => Err(value),
            },
            None => Err(value),
        }?;

        Ok(Monkey {
            id,
            items,
            operation,
            test: Test {
                divisble_by,
                if_true,
                if_false,
            },
            inspection_count: 0,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct MonkeyItemUpdate {
    throw_to: usize,
    worry_level: i64,
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<MonkeyItemUpdate> {
        let mut updates: Vec<MonkeyItemUpdate> = Vec::with_capacity(self.items.len());

        while let Some(item) = self.items.pop() {
            let updated_worry_level = match self.operation.arguments {
                OperationArgument::OldAndOther(other) => self.operation.compute(item, other) / 3,
                OperationArgument::OldAndOld => self.operation.compute(item, item) / 3,
            };
            let throw_to = self.test.throw_to(updated_worry_level);

            self.inspection_count += 1;

            updates.push(MonkeyItemUpdate {
                throw_to,
                worry_level: updated_worry_level,
            });
        }

        updates.iter().rev().copied().collect_vec()
    }
}
