use crate::errors::Error;

enum FileSize {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

#[derive(Debug)]
pub struct Size {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

struct SizeAndUnitParsed(u64, FileSize);
struct ComputedSize(u64, u64, u64, u64);

pub struct FileSizeFormatter;

impl FileSizeFormatter {
    pub fn new(size_and_unit: String) -> Result<Size, Error> {
        let result = Self.parse(size_and_unit)?;
        let (size, unit) = (result.0, result.1);
        let result = Self.compute(size, unit);
        let (bytes, kilobytes, megabytes, gigabytes) = (result.0, result.1, result.2, result.3);

        Ok(Size {
            bytes: format!("{bytes} bytes"),
            kilobytes: format!("{kilobytes} kilobytes"),
            megabytes: format!("{megabytes} megabytes"),
            gigabytes: format!("{gigabytes} gigabytes"),
        })
    }

    fn parse(&self, size_and_unit: String) -> Result<SizeAndUnitParsed, Error> {
        let size_and_unit: Vec<&str> = size_and_unit.split_whitespace().collect();

        if size_and_unit.len() != 2 {
            return Err(Error::InvalidParsedSizeAndUnit(
                "InvalidParsedSizeAndUnit : {size_and_unit}".to_owned(),
            ));
        }

        let (size, unit) = (size_and_unit[0], size_and_unit[1]);

        let size: u64 = match size.parse::<u64>() {
            Ok(size) => size,
            Err(_) => {
                return Err(Error::InvalidParsedSize(
                    "InvalidParsedSize : {size}".to_owned(),
                ))
            }
        };

        let unit: FileSize = match unit {
            "b" | "B" | "bytes" | "Bytes" => FileSize::Bytes,
            "kb" | "KB" | "kilobytes" | "Kilobytes" => FileSize::Kilobytes,
            "mb" | "MB" | "megabytes" | "Megabytes" => FileSize::Megabytes,
            "gb" | "GB" | "gigabytes" | "Gigabytes" => FileSize::Gigabytes,
            _ => {
                return Err(Error::InvalidParsedUnit(
                    "InvalidParsedUnit : {unit}".to_owned(),
                ))
            }
        };

        Ok(SizeAndUnitParsed(size, unit))
    }

    fn compute(&self, size: u64, unit: FileSize) -> ComputedSize {
        let byte_factor = 1000; // Or possible 1024
        let mut bytes = 0;
        let mut kilobytes = 0;
        let mut megabytes = 0;
        let mut gigabytes = 0;

        match unit {
            FileSize::Bytes => {
                bytes = size;
                kilobytes = size / byte_factor;
                megabytes = size / byte_factor / byte_factor;
                gigabytes = size / byte_factor / byte_factor / byte_factor;
            }
            FileSize::Kilobytes => {
                bytes = size * byte_factor;
                kilobytes = size;
                megabytes = size / byte_factor;
                gigabytes = size / byte_factor / byte_factor;
            }
            FileSize::Megabytes => {
                bytes = size * byte_factor * byte_factor;
                kilobytes = size * byte_factor;
                megabytes = size;
                gigabytes = size / byte_factor;
            }
            FileSize::Gigabytes => {
                bytes = size * byte_factor * byte_factor * byte_factor;
                kilobytes = size * byte_factor * byte_factor;
                megabytes = size * byte_factor;
                gigabytes = size;
            }
        }

        ComputedSize(bytes, kilobytes, megabytes, gigabytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_size_formatter() {
        let size = FileSizeFormatter::new("1000 bytes".to_owned()).unwrap();
        assert_eq!(size.bytes, "1000 bytes");
        assert_eq!(size.kilobytes, "1 kilobytes");
        assert_eq!(size.megabytes, "0 megabytes");
        assert_eq!(size.gigabytes, "0 gigabytes");

        let size = FileSizeFormatter::new("1000 kilobytes".to_owned()).unwrap();
        assert_eq!(size.bytes, "1000000 bytes");
        assert_eq!(size.kilobytes, "1000 kilobytes");
        assert_eq!(size.megabytes, "1 megabytes");
        assert_eq!(size.gigabytes, "0 gigabytes");

        let size = FileSizeFormatter::new("1000 megabytes".to_owned()).unwrap();
        assert_eq!(size.bytes, "1000000000 bytes");
        assert_eq!(size.kilobytes, "1000000 kilobytes");
        assert_eq!(size.megabytes, "1000 megabytes");
        assert_eq!(size.gigabytes, "1 gigabytes");

        let size = FileSizeFormatter::new("1000 gigabytes".to_owned()).unwrap();
        assert_eq!(size.bytes, "1000000000000 bytes");
        assert_eq!(size.kilobytes, "1000000000 kilobytes");
        assert_eq!(size.megabytes, "1000000 megabytes");
        assert_eq!(size.gigabytes, "1000 gigabytes");
    }

    #[test]
    fn test_file_size_formatter_invalid_parsed_size_and_unit() {
        let size = FileSizeFormatter::new("1000".to_owned());
        assert!(size.is_err());

        if let Err(Error::InvalidParsedSizeAndUnit(msg)) = size {
            assert_eq!(msg, "InvalidParsedSizeAndUnit : {size_and_unit}");
        } else {
            panic!("Expected InvalidParsedSizeAndUnit error");
        }
    }

    #[test]
    fn test_file_size_formatter_invalid_parsed_size() {
        let size = FileSizeFormatter::new("1000.0 bytes".to_owned());
        assert!(size.is_err());

        if let Err(Error::InvalidParsedSize(msg)) = size {
            assert_eq!(msg, "InvalidParsedSize : {size}");
        } else {
            panic!("Expected InvalidParsedSize error");
        }
    }

    #[test]
    fn test_file_size_formatter_invalid_parsed_unit() {
        let size = FileSizeFormatter::new("1000.0 bytes".to_owned());
        assert!(size.is_err());

        if let Err(Error::InvalidParsedSize(msg)) = size {
            assert_eq!(msg, "InvalidParsedSize : {size}");
        } else {
            panic!("Expected InvalidParsedSize error");
        }
    }
}
