use itertools::Itertools;
const DIRECTIONS: [SightDirection; 4] = [
    SightDirection::Up,
    SightDirection::Down,
    SightDirection::Left,
    SightDirection::Right,
];

pub fn part1() {
    let input = include_str!("input.txt");

    let grid = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().map(|c| c as u8).collect_vec())
        .collect_vec();

    let mut visible_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if grid.is_position_visible(Position::new(x, y)) {
                visible_count += 1;
            }
        }
    }

    println!("{visible_count}");
}

pub fn part2() {
    let input = include_str!("input.txt");

    let grid = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().map(|c| c as u8).collect_vec())
        .collect_vec();

    let mut max_scenic_score = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            let scenic_score = grid.scenic_score_for_position(Position::new(x, y));

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("{max_scenic_score}");
}

#[derive(Clone, Copy)]
enum SightDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn advance(&self, direction: SightDirection, grid: &Vec<Vec<u8>>) -> Option<Self> {
        match direction {
            SightDirection::Up => (self.y > 0).then(|| Position::new(self.x, self.y - 1)),
            SightDirection::Down => {
                (self.y + 1 < grid.len()).then(|| Position::new(self.x, self.y + 1))
            }
            SightDirection::Left => {
                (self.x + 1 < grid[0].len()).then(|| Position::new(self.x + 1, self.y))
            }
            SightDirection::Right => (self.x > 0).then(|| Position::new(self.x - 1, self.y)),
        }
    }
}

trait Forest {
    fn get_entry_point(&self, x: usize, y: usize, direction: SightDirection) -> Position;
    fn is_position_visible(&self, position: Position) -> bool;
    fn scenic_score_for_position(&self, position: Position) -> u64;
}

impl Forest for Vec<Vec<u8>> {
    fn get_entry_point(&self, x: usize, y: usize, direction: SightDirection) -> Position {
        match direction {
            SightDirection::Up => Position::new(x, self.len() - 1),
            SightDirection::Down => Position::new(x, 0),
            SightDirection::Left => Position::new(0, y),
            SightDirection::Right => Position::new(self[0].len() - 1, y),
        }
    }

    fn scenic_score_for_position(&self, position: Position) -> u64 {
        let mut score = 1;
        let height = self[position.y][position.x];

        for direction in DIRECTIONS {
            let edge_position = self.get_entry_point(position.x, position.y, direction);
            let mut current_position = position.clone();
            let mut current_position_score = 0;

            while !(current_position.x == edge_position.x && current_position.y == edge_position.y)
            {
                match current_position.advance(direction, self) {
                    Some(pos) => {
                        current_position_score += 1;
                        current_position = pos;
                        if self[current_position.y][current_position.x] >= height {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            score *= current_position_score;
        }

        score
    }

    fn is_position_visible(&self, position: Position) -> bool {
        let mut visible = false;
        let height = self[position.y][position.x];

        for direction in DIRECTIONS {
            let mut visible_from_direction = true;
            let entry_point = self.get_entry_point(position.x, position.y, direction);

            let mut current_position = entry_point.clone();

            while !(current_position.x == position.x && current_position.y == position.y) {
                if self[current_position.y][current_position.x] >= height {
                    visible_from_direction = false;
                    break;
                }
                current_position = current_position
                    .advance(direction, self)
                    .expect("Should not be able to get outside grid");
                visible_from_direction = true;
            }

            visible |= visible_from_direction;
        }

        return visible;
    }
}
