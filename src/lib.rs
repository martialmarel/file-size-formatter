mod command_line;
mod errors;
mod file_size_formatter;

use command_line::CommandLine;
use errors::Error;
use file_size_formatter::FileSizeFormatter;

pub fn process() -> Result<(), Error> {
    let command_line = CommandLine::parse();

    let size_and_unit = match command_line.get_size_and_unit() {
        Some(size_and_unit) => size_and_unit,
        None => {
            return Err(Error::NotArgumentSizeAndUnitProvided(String::from(
                "NotArgumentSizeAndUnitProvided",
            )))
        }
    };

    let size = FileSizeFormatter::new(size_and_unit);
    println!("{:?}", size);

    Ok(())
}
