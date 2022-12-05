use itertools::Itertools;

pub fn part1() -> Vec<char> {
    let input = include_str!("input.txt");

    let (drawing, moves) = input
        .splitn(2, "\n\n")
        .collect_tuple()
        .expect("No separator between drawing and moves list.");

    let stacks = parse_stacks_from_drawing(drawing);
    let mut ship = Ship {
        crate_stacks: stacks,
    };

    let moves = parse_moves(moves);

    for mv in moves.iter() {
        if ship.move_crates(*mv).is_err() {
            println!("Move: {:?} failed.", mv);
        }
    }

    ship.crate_stacks
        .iter_mut()
        .filter_map(|s| s.pop())
        .map(|c| c.symbol)
        .collect_vec()
}

pub fn parse_stacks_from_drawing(drawing: &str) -> Vec<Vec<Crate>> {
    let mut lines_iter = drawing.lines().rev();

    let mut stacks: Vec<Vec<Crate>> = lines_iter
        .next()
        .expect("No lines in drawing string.")
        .split(' ')
        .filter(|s| s.parse::<i64>().is_ok())
        .map(|_| Vec::new())
        .collect_vec();

    println!("{:?}", stacks);

    for line in lines_iter {
        for (i, c) in line.chars().enumerate() {
            let idx = if i > 0 { (i - 1) / 4 } else { 0 };
            if i % 4 == 1 && c != ' ' {
                stacks[idx].push(Crate { symbol: c });
            }
        }
    }

    stacks
}

pub fn parse_moves(moves_str: &str) -> Vec<Move> {
    moves_str
        .lines()
        .flat_map(|line| {
            line.split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .tuples()
                .map(|(amount, from, to)| Move::new((from - 1) as usize, (to - 1) as usize, amount))
        })
        .collect_vec()
}

pub struct Ship {
    crate_stacks: Vec<Vec<Crate>>,
}

impl Ship {
    pub fn move_crates(&mut self, mv: Move) -> Result<u64, ()> {
        let mut crates_to_move: Vec<Crate> = Vec::with_capacity(mv.amount as usize);

        if let Some(from_stack) = self.crate_stacks.get_mut(mv.from) {
            for _ in 0..mv.amount {
                if let Some(crate_to_move) = from_stack.pop() {
                    crates_to_move.push(crate_to_move);
                } else {
                    println!("wut");
                    return Err(());
                }
            }
            for x in self.crate_stacks.clone() {
                println!("{}", x.len());
            }
        } else {
            return Err(());
        }

        if let Some(to_stack) = self.crate_stacks.get_mut(mv.to) {
            for c in crates_to_move {
                to_stack.push(c);
            }
        }

        Ok(mv.amount)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Crate {
    symbol: char,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    from: usize,
    to: usize,
    amount: u64,
}

impl Move {
    pub fn new(from: usize, to: usize, amount: u64) -> Self {
        Move { from, to, amount }
    }
}
