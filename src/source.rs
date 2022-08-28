use crate::config::{Config, SourceType};

#[derive(PartialEq, Debug)]
pub struct Source(String);

impl Source {
    pub fn from_config (config: Config) -> Result<Source, &'static str> {
        match config.source_type {
            SourceType::Raw => Ok(Source(config.source)),
            SourceType::File => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_source_from_config_if_type_is_raw() {
        let config = Config { source: "[.]".to_string(), source_type: SourceType::Raw };

        assert_eq!(Source::from_config(config).unwrap(), Source("[.]".to_string()));
    }
}