use itertools::Itertools;
use serde::Deserialize;
use std::{cmp::Ordering, fmt::Debug};

pub fn part1() -> usize {
    let input = include_str!("input.txt");

    let mut sum = 0;
    for (i, groups) in input.split("\n\n").enumerate() {
        let i = i + 1;

        let mut nodes = groups
            .lines()
            .map(|line| serde_json::from_str::<ListNode>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();
        println!("\n== Pair {i} ==");
        println!("l = {l:?}");
        println!("r = {r:?}");
        println!("l < r = {}", l < r);
        if l < r {
            sum += i;
        }
    }

    sum
}

pub fn part2() -> usize {
    let input = include_str!("input.txt");
    let dividers = vec![
        ListNode::List(vec![ListNode::Value(2)]),
        ListNode::List(vec![ListNode::Value(6)]),
    ];

    let mut packets = include_str!("input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<ListNode>(line).unwrap())
        .chain(dividers.iter().cloned())
        .sorted()
        .collect::<Vec<_>>();

    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    decoder_key
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut current_val: Option<u32> = None;

    let mut tokens: Vec<Token> = vec![];

    for c in input.chars() {
        match c {
            '[' => tokens.push(Token::ListStart),
            ']' => {
                if let Some(val) = current_val {
                    tokens.push(Token::Value(val));
                    current_val = None;
                }
                tokens.push(Token::ListEnd);
            }
            ',' => {
                if let Some(val) = current_val {
                    tokens.push(Token::Value(val));
                    current_val = None;
                }
            }
            _ => {
                if let Some(n) = c.to_digit(10) {
                    match current_val {
                        Some(val) => {
                            current_val = Some((val * 10) + n);
                        }
                        None => {
                            current_val = Some(n);
                        }
                    }
                }
            }
        }
    }

    tokens
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(untagged)]
enum ListNode {
    Value(u32),
    List(Vec<ListNode>),
}

impl ListNode {
    fn as_list<T>(&self, f: impl FnOnce(&[ListNode]) -> T) -> T {
        match self {
            ListNode::Value(val) => f(&[Self::Value(*val)]),
            ListNode::List(l) => f(&l[..]),
        }
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ListNode::Value(a), ListNode::Value(b)) => a.partial_cmp(b),
            (left, right) => Some(left.as_list(|l| {
                right.as_list(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(node1, node2)| node1.cmp(node2))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    ListStart,
    ListEnd,
    Value(u32),
}
