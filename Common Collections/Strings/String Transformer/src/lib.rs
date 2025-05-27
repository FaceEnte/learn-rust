pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    // TODO: Complete the function signature!
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, command) in input {
            output.push(match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(size) => format!("{}{}", string, "bar".repeat(size)),
            })
        }
        output
    }
}
