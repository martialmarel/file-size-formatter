use std::env;

pub struct CommandLine {
    pub args: Vec<String>,
}

impl CommandLine {
    pub fn parse() -> CommandLine {
        CommandLine {
            args: env::args().collect(),
        }
    }

    fn get_arg(&self, index: usize) -> String {
        self.args[index].clone()
    }

    /// Returns the path of the executable.
    pub fn get_executable_path(&self) -> String {
        self.get_arg(0)
    }

    /// get arg a String representing size and unit
    pub fn get_size_and_unit(&self) -> Option<String> {
        if self.args.len() == 1 {
            return None;
        }

        Some(self.get_arg(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_executable_path() {
        let command_line = CommandLine {
            args: vec![String::from("target/debug/du"), String::from("1 mb")],
        };

        assert_eq!(
            command_line.get_executable_path(),
            String::from("target/debug/du")
        );
    }

    #[test]
    fn test_get_size_and_unit() {
        let command_line = CommandLine {
            args: vec![String::from("target/debug/du"), String::from("16 mb")],
        };

        assert_eq!(
            command_line.get_size_and_unit(),
            Some(String::from("16 mb"))
        );
    }

    #[test]
    fn test_get_size_and_unit_no_args() {
        let command_line = CommandLine {
            args: vec![String::from("target/debug/du")],
        };

        assert_eq!(command_line.get_size_and_unit(), None);
    }
}
