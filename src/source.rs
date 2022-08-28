use std::io;

use crate::config::{Config, SourceType};

type FsReadToString = fn(String) -> io::Result<String>;

#[derive(PartialEq, Debug)]
pub struct Source(String);

impl Source {
    pub fn from_config(config: Config, fs_read_to_string: FsReadToString) -> Result<Source, &'static str> {
        let source = match config.source_type {
            SourceType::Raw => config.source,
            SourceType::File => fs_read_to_string(config.source).unwrap(),
        };

        Ok(Source(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_fs_read_to_string(_: String) -> io::Result<String> {
        io::Result::Ok("__mock".to_string())
    }

    #[test]
    fn it_should_return_source_from_config_if_type_is_raw() {
        let config = Config {
            source: "[.]".to_string(),
            source_type: SourceType::Raw,
        };

        assert_eq!(
            Source::from_config(config, mock_fs_read_to_string).unwrap(),
            Source("[.]".to_string())
        );
    }

    #[test]
    fn it_should_read_source_from_fs_if_type_is_file() {
        let config = Config {
            source: "filename.bf".to_string(),
            source_type: SourceType::File,
        };

        assert_eq!(
            Source::from_config(config, mock_fs_read_to_string).unwrap(),
            Source("__mock".to_string())
        );
    }
}
