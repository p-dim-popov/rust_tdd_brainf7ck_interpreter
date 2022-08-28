#[derive(Debug)]
#[derive(PartialEq)]
pub enum SourceType {
    Raw, File,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub source: String,
    pub source_type: SourceType,
}

impl Config {
    pub fn parse(mut input: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        input.next();

        let src = match input.next() {
            Some(a) => a,
            _ => return Err("No source given"),
        };

        Ok(Config {source: src.to_string(), source_type: SourceType::File})
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn it_should_parse_minimal_variant_as_file() {
        expect_input_to_be_parsed_correctly(
            "bf hello-world.bf", 
            Config { source: "hello-world.bf".to_string(), source_type: SourceType::File }
        )
    }

    fn expect_input_to_be_parsed_correctly(input: &'static str, expected: Config) {
        let input = input.split_ascii_whitespace().map(String::from);

        let result = Config::parse(input).unwrap();

        assert_eq!(result, expected)        
    }
}
