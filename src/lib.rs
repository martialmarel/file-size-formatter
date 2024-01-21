mod command_line;
mod errors;

use command_line::CommandLine;
use errors::Error;

pub fn process() -> Result<(), Error> {
    let command_line = CommandLine::parse();

    match command_line.get_size_and_unit() {
        Some(size_and_unit) => println!("size_and_unit: {:?}", size_and_unit),
        None => {
            return Err(Error::NotArgumentSizeAndUnitProvided(String::from(
                "NotArgumentSizeAndUnitProvided",
            )))
        }
    }

    Ok(())
}
