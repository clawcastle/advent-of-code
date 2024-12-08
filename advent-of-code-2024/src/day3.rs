pub fn part1() {
    let input = parse_input();

    let result: i32 = input.instructions.iter().map(|instruction| instruction.execute()).sum();

    println!("Day 1 part 1 solution: {}", result);
}

fn parse_input() -> Input {
    let file_content = std::fs::read_to_string("./input/day3.txt").expect("Couldn't find input file.");

    let tokens = tokenize(&file_content);
    let instructions = parse_token_stream(&tokens);

    Input { instructions }
}

fn valid_next_token(prev: Option<&Token>, next: &Token) -> bool {
    match prev {
        Some(Token::Mul) => matches!(next, Token::LeftParen),
        Some(Token::LeftParen) => matches!(next, Token::Number(_)),
        Some(Token::RightParen) => false,
        Some(Token::Number(_)) => matches!(next, Token::Comma) || matches!(next, Token::RightParen),
        Some(Token::Comma) => matches!(next, Token::Number(_)),
        Some(Token::InvalidCharacter) => false,
        None => true
    }
}

fn parse_instruction(tokens: &[Token]) -> Option<Instruction> {
    match tokens {
        [Token::Mul, Token::LeftParen, Token::Number(n1), Token::Comma, Token::Number(n2), Token::RightParen] => if *n1 < 1000 && *n2 < 1000 {
            Some(Instruction::Multiply(*n1, *n2))
        } else {
            None
        },
        _ => None
    }
}

fn parse_token_stream(tokens: &[Token]) -> Vec<Instruction> {
    let mut buffer: Vec<Token> = Vec::new();

    let mut instructions: Vec<Instruction> = Vec::new();

    for token in tokens {
        if !valid_next_token(buffer.last(), token) {
            if let Some(instruction) = parse_instruction(&buffer) {
                instructions.push(instruction);
            }

            buffer.clear();
        }

        buffer.push(*token);
    }

    if let Some(instruction) = parse_instruction(&buffer) {
        instructions.push(instruction);
    }

    println!("instructions: {:?}", &instructions);
    instructions
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut buffer = String::new();

    let mut tokens: Vec<Token> = Vec::new();

    // xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    for c in s.chars() {
        if !Token::valid_next_char(&buffer, c) {
            tokens.push(Token::from_str(&buffer));

            buffer.clear();
        }

        buffer.push(c);
    }

    tokens.push(Token::from_str(&buffer));

    println!("{:?}", &tokens);

    tokens
}

struct Input {
    instructions: Vec<Instruction>
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Multiply(i32, i32)
}

impl Instruction {
    fn execute(&self) -> i32 {
        match self {
            Instruction::Multiply(n1, n2) => n1 * n2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Mul,
    LeftParen,
    RightParen,
    Comma,
    InvalidCharacter,
    Number(i32),
}

impl Token {
    fn from_str(s: &str) -> Token {
        match s {
            "mul" => Token::Mul,
            "(" => Token::LeftParen,
            "," => Token::Comma,
            ")" => Token::RightParen,
            " " => Token::InvalidCharacter,
            s if s.chars().all(|c| c.is_numeric()) => s.parse::<i32>().map(|n| Token::Number(n)).unwrap_or(Token::InvalidCharacter),
            _ => Token::InvalidCharacter
        }
    }

    fn valid_next_char(buffer: &str, next: char) -> bool {
        let mut s = String::from(buffer);
        s.push(next);

        match s.as_str() {
            "(" | "," | ")" | " " => false, // single-character tokens
            s if "mul".starts_with(s) || (s.chars().all(|c| c.is_numeric())) => true, // part of multi-character tokens
            _ => false
        }
    }
}