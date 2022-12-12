use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");

    let x = input
        .lines()
        .filter_map(|line| Either::try_from(line).ok())
        .collect_vec();
}

enum Either<'a> {
    Command(Command<'a>),
    FileSystemType(FileSystemType),
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
}
enum FileSystemType {
    File(u64),
    Dir,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match &value[..2] {
            "ls" => Ok(Command::Ls),
            "cd" => Ok(Command::Cd(&value[3..])),
            _ => Err(value),
        }
    }
}

impl TryFrom<&str> for FileSystemType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.splitn(2, ' ').collect_vec();

        if parts.len() < 2 {
            return Err(value.to_string());
        }

        if parts[0] == "dir" {
            return Ok(FileSystemType::Dir);
        }

        if let Ok(file_size) = parts[0].parse::<u64>() {
            return Ok(FileSystemType::File(file_size));
        }

        Err(value.to_string())
    }
}

impl<'a> TryFrom<&'a str> for Either<'a> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Ok(command) = Command::try_from(value) {
            return Ok(Either::Command(command));
        }
        if let Ok(file_system_type) = FileSystemType::try_from(value) {
            return Ok(Either::FileSystemType(file_system_type));
        }

        Err(value)
    }
}
