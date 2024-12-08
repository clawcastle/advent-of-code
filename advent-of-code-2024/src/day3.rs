pub fn part1() {
    let tokens = parse_input();
    
    let token_parser = TokenParser { enable_dos_and_donts: false };

    let instructions = token_parser.parse_token_stream(&tokens);

    let mut program = Program::new(instructions);

    let result: i32 = program.execute();

    println!("Day 1 part 1 solution: {}", result);
}

pub fn part2() {
    let tokens = parse_input();
    
    let token_parser = TokenParser { enable_dos_and_donts: true };

    let instructions = token_parser.parse_token_stream(&tokens);

    let mut program = Program::new(instructions);
    
    let result: i32 = program.execute();

    println!("Day 1 part 1 solution: {}", result);
}

fn parse_input() -> Vec<Token> {
    let file_content = std::fs::read_to_string("./input/day3.txt").expect("Couldn't find input file.");

    let tokens = tokenize(&file_content);

    tokens
}

struct TokenParser {
    enable_dos_and_donts: bool,
}

impl TokenParser {
    fn valid_next_token(&self, prev: Option<&Token>, next: &Token) -> bool {
        match prev {
            Some(Token::Mul) => matches!(next, Token::LeftParen),
            Some(Token::LeftParen) => matches!(next, Token::Number(_)),
            Some(Token::RightParen) => false,
            Some(Token::Number(_)) => matches!(next, Token::Comma) || matches!(next, Token::RightParen),
            Some(Token::Comma) => matches!(next, Token::Number(_)),
            Some(Token::InvalidCharacter) => false,
            Some(Token::Do) => !self.enable_dos_and_donts,
            Some(Token::Dont) => !self.enable_dos_and_donts,
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
            [Token::Do] => Some(Instruction::Enable),
            [Token::Dont] => Some(Instruction::Disable),
            _ => None
        }
    }
    
    fn parse_token_stream(&self, tokens: &[Token]) -> Vec<Instruction> {
        let mut buffer: Vec<Token> = Vec::new();
    
        let mut instructions: Vec<Instruction> = Vec::new();
    
        for token in tokens {
            if !self.valid_next_token(buffer.last(), token) {
                if let Some(instruction) = TokenParser::parse_instruction(&buffer) {
                    instructions.push(instruction);
                }
    
                buffer.clear();
            }
    
            buffer.push(*token);
        }
    
        if let Some(instruction) = TokenParser::parse_instruction(&buffer) {
            instructions.push(instruction);
        }
    
        instructions
    }
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

    tokens
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Multiply(i32, i32),
    Enable,
    Disable
}

struct Program {
    instructions: Vec<Instruction>,
    multiplication_enabled: bool
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            multiplication_enabled: true
        }
    }

    fn execute(&mut self) -> i32 {
        let mut result = 0;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Enable => self.multiplication_enabled = true,
                Instruction::Disable => self.multiplication_enabled = false,
                Instruction::Multiply(n1, n2) => if self.multiplication_enabled {
                    result += n1 * n2
                }
            }
        }

        result
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
    Do,
    Dont
}

impl Token {
    fn from_str(s: &str) -> Token {
        match s {
            "mul" => Token::Mul,
            "(" => Token::LeftParen,
            "," => Token::Comma,
            ")" => Token::RightParen,
            " " => Token::InvalidCharacter,
            "do()" => Token::Do,
            "don't()" => Token::Dont,
            s if s.chars().all(|c| c.is_numeric()) => s.parse::<i32>().map(|n| Token::Number(n)).unwrap_or(Token::InvalidCharacter),
            _ => Token::InvalidCharacter
        }
    }

    fn valid_next_char(buffer: &str, next: char) -> bool {
        let mut s = String::from(buffer);
        s.push(next);

        match s.as_str() {
            s if "do()".starts_with(s) || "don't()".starts_with(s) => true,
            "(" | "," | ")" | " " => false, // single-character tokens
            s if "mul".starts_with(s) || (s.chars().all(|c| c.is_numeric())) => true, // part of multi-character tokens
            _ => false
        }
    }
}