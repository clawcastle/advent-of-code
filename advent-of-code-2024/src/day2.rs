use iter_tools::Itertools;

pub fn part1() -> () {
    let input = parse_input();

    let n_safe_reports = input.lines.iter().filter(|line| line.is_safe()).count();

    println!("Day 2 part 1 solution: {}", n_safe_reports);
}

pub fn part2() -> () {
    let input = parse_input();

    let n_safe_reports = input.lines.iter().filter(|line| line.is_safe_with_dampening(1)).count();

    println!("Day 2 part 2 solution: {}", n_safe_reports);
}

fn parse_input() -> Input {
    let file_content = std::fs::read_to_string("./input/day2.txt").expect("Couldn't find input file.");

    let lines = file_content.lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i32>().expect("Failed to parse number.")))
        .map(|line| InputLine(line.collect()))
        .collect();

    Input {
        lines
    }
}

struct Input {
    lines: Vec<InputLine>
}

struct InputLine(Vec<i32>);

impl InputLine {
    fn is_safe(&self) -> bool {
        let valid_decreasing = self.0.iter().tuple_windows().all(|(a,b)| a > b && (a - b > 0 && a - b < 4));
        let valid_increasing = self.0.iter().tuple_windows().all(|(a,b)| a < b && (b - a > 0 && b - a < 4));

        valid_decreasing || valid_increasing
    }

    fn is_safe_with_dampening(&self, allow_skipping_n_levels: usize) -> bool {
        if self.0.len() < 2 {
            return true;
        }

        let increasing = self.0[0] < self.0[1];

        let mut prev = self.0[0];
        let first_diff = (self.0[1] - prev).abs();
        let mut n_skips = if first_diff > 0 && first_diff < 4 {
            0
        } else {
            1
        };

        for n in self.0.iter().skip(n_skips + 1) {
            let diff = if increasing {
                n - prev
            } else {
                prev - n 
            };

            let valid_diff = diff > 0 && diff < 4;

            if valid_diff {
                prev = *n;
                continue;
            }

            if !valid_diff && n_skips < allow_skipping_n_levels {
                n_skips += 1;
            } else {
                return false;
            }
        }

        return true;
    }
}

