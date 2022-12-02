use itertools::Itertools;

pub fn part1() -> u64 {
    let input = include_str!("input.txt");

    input
        .lines()
        .filter_map(|line| MatchRound::from_input_line_part_1(line))
        .map(|match_round| play_round(match_round).score())
        .sum()
}

pub fn part2() -> u64 {
    let input = include_str!("input.txt");

    input
        .lines()
        .filter_map(|line| MatchRound::from_input_line_part_2(line))
        .map(|match_round| play_round(match_round).score())
        .sum()
}

fn play_round(match_round: MatchRound) -> MatchOutcome {
    match (&match_round.me, &match_round.opponent) {
        (Shape::Rock, Shape::Rock) => MatchOutcome::Draw(match_round.me),
        (Shape::Rock, Shape::Paper) => MatchOutcome::Loss(match_round.me),
        (Shape::Rock, Shape::Scissor) => MatchOutcome::Win(match_round.me),
        (Shape::Paper, Shape::Rock) => MatchOutcome::Win(match_round.me),
        (Shape::Paper, Shape::Paper) => MatchOutcome::Draw(match_round.me),
        (Shape::Paper, Shape::Scissor) => MatchOutcome::Loss(match_round.me),
        (Shape::Scissor, Shape::Rock) => MatchOutcome::Loss(match_round.me),
        (Shape::Scissor, Shape::Paper) => MatchOutcome::Win(match_round.me),
        (Shape::Scissor, Shape::Scissor) => MatchOutcome::Draw(match_round.me),
    }
}

pub struct MatchRound {
    me: Shape,
    opponent: Shape,
}

impl MatchRound {
    pub fn new(my_shape: Shape, opponent_shape: Shape) -> Self {
        return MatchRound {
            me: my_shape,
            opponent: opponent_shape,
        };
    }
}

impl MatchRound {
    pub fn from_input_line_part_1(line: &str) -> Option<Self> {
        let chars = line.chars().collect_vec();

        if let Some(opponent_shape_char) = chars.get(0) && let Some(my_shape_char) = chars.get(2) {
            if let Some(opponent_shape) = Shape::from_char(*opponent_shape_char) && let Some(my_shape) = Shape::from_char(*my_shape_char) {
                return Some(MatchRound::new(my_shape, opponent_shape));
            }
        }

        None
    }

    pub fn from_input_line_part_2(line: &str) -> Option<Self> {
        let chars = line.chars().collect_vec();

        if let Some(opponent_shape_char) = chars.get(0) && let Some(my_shape_char) = chars.get(2) {
            if let Some(opponent_shape) = Shape::from_char(*opponent_shape_char) && let Some(my_shape) = Shape::from_char_and_opponents_shape(*my_shape_char, opponent_shape) {
                return Some(MatchRound::new(my_shape, opponent_shape));
            }
        }

        None
    }
}

#[derive(Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

pub enum MatchOutcome {
    Win(Shape),
    Loss(Shape),
    Draw(Shape),
}

impl MatchOutcome {
    pub fn score(&self) -> u64 {
        match self {
            MatchOutcome::Win(winning_shape) => 6 + winning_shape.score(),
            MatchOutcome::Draw(draw_shape) => 3 + draw_shape.score(),
            MatchOutcome::Loss(losing_shape) => 0 + losing_shape.score(),
        }
    }
}

impl Shape {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissor),
            _ => None,
        }
    }

    pub fn from_char_and_opponents_shape(c: char, opponent_shape: Shape) -> Option<Self> {
        match c {
            'X' => Some(opponent_shape.results_in_win()),
            'Y' => Some(opponent_shape.results_in_draw()),
            'Z' => Some(opponent_shape.results_in_loss()),
            _ => None,
        }
    }

    pub fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    pub fn results_in_loss(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    pub fn results_in_draw(&self) -> Self {
        match self {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissor => Shape::Scissor,
        }
    }

    pub fn results_in_win(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }
}
