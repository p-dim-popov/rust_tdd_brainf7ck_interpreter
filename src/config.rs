#[derive(Debug, PartialEq)]
pub enum SourceType {
    Raw,
    File,
}

impl SourceType {
    fn from(x: &str) -> Option<SourceType> {
        match x {
            "raw" => Some(SourceType::Raw),
            "file" => Some(SourceType::File),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub source: String,
    pub source_type: SourceType,
}

impl Config {
    pub fn parse(mut input: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        input.next();

        let source = match input.next() {
            Some(a) => a,
            _ => return Err("No source given"),
        };

        let source_type = match input.next() {
            Some(x) => match x.split_once("=") {
                Some((key, val)) => match key {
                    "--source-type" => match SourceType::from(val) {
                        Some(st) => st,
                        _ => return Err("Unknown source type"),
                    },
                    _ => return Err("Unknown option")
                },
                None => return Err("Invalid option"),
            },
            _ => SourceType::File,
        };

        Ok(Config {
            source,
            source_type,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn it_should_parse_minimal_variant_as_file() {
        expect_input_to_be_parsed_correctly(
            "bf hello-world.bf",
            Config {
                source: "hello-world.bf".to_string(),
                source_type: SourceType::File,
            },
        )
    }

    #[test]
    fn it_should_parse_with_source_type_flag() {
        expect_input_to_be_parsed_correctly(
            "bf [.] --source-type=raw",
            Config {
                source: "[.]".to_string(),
                source_type: SourceType::Raw,
            },
        );

        expect_input_to_be_parsed_correctly(
            "bf hello.bf --source-type=file",
            Config {
                source: "hello.bf".to_string(),
                source_type: SourceType::File,
            },
        );
    }

    fn expect_input_to_be_parsed_correctly(input: &'static str, expected: Config) {
        let input = input.split_ascii_whitespace().map(String::from);

        let result = Config::parse(input).unwrap();

        assert_eq!(result, expected)
    }
}
