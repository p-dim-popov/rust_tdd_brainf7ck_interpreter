use std::io;

use crate::config::{Config, SourceType};

type FsReadToString = fn(String) -> io::Result<String>;

#[derive(PartialEq, Debug)]
pub struct Source(String);

pub struct SourceAdapter {
    fs_read_to_string: FsReadToString,
}

impl SourceAdapter {
    pub fn new(fs_read_to_string: FsReadToString) -> SourceAdapter {
        SourceAdapter { fs_read_to_string }
    }

    pub fn from_config(&self, config: Config) -> Result<Source, &'static str> {
        let source = match config.source_type {
            SourceType::Raw => config.source,
            SourceType::File => (self.fs_read_to_string)(config.source).unwrap(),
        };

        Ok(Source(source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mock_source_adapter() -> SourceAdapter {
        fn mock_fs_read_to_string(_: String) -> io::Result<String> {
            io::Result::Ok("__mock".to_string())
        }

        SourceAdapter {
            fs_read_to_string: mock_fs_read_to_string,
        }
    }

    #[test]
    fn it_should_return_source_from_config_if_type_is_raw() {
        let config = Config {
            source: "[.]".to_string(),
            source_type: SourceType::Raw,
        };

        assert_eq!(
            get_mock_source_adapter().from_config(config).unwrap(),
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
            get_mock_source_adapter().from_config(config).unwrap(),
            Source("__mock".to_string())
        );
    }
}
