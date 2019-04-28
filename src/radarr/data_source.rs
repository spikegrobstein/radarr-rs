use clap::ArgMatches;
use std::error::Error;

pub enum DataSource {
    Stdin,
    File(String),
    Data(String),
}

impl DataSource {
    pub fn from_matches(matches: &ArgMatches) -> Option<DataSource> {
        let file_path = matches.value_of("file");
        let data = matches.value_of("data");

        DataSource::from(file_path, data)
    }

    pub fn from(file_path: Option<&str>, data: Option<&str>) -> Option<DataSource> {
        match (file_path, data) {
            (Some(_), Some(_)) => {
                // eprintln!("nothing.");
                None
            },
            (None, Some(data)) => {
                // got raw data
                // eprintln!("passed data");
                Some(DataSource::Data(String::from(data)))
            },
            (file_path, None) => {
                let file_path = file_path.unwrap_or("-");

                if file_path == "-" {
                    // read from stdin 
                    // eprintln!("stdin!");
                    Some(DataSource::Stdin)
                } else {
                    // got just a file path
                    // eprintln!("file");
                    Some(DataSource::File(String::from(file_path)))
                }
            },
        }
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        match self {
            DataSource::Data(data) => Ok(data.to_string()),
            DataSource::File(file_path) => {
                Ok(file_path.to_string())
            },
            DataSource::Stdin => {
                Ok(String::from("stdin"))
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_does_not_allow_both_args() {
        let result = DataSource::from(Some("foo"), Some("bar"));

        assert!(result.is_none())
    }

    #[test]
    fn from_returns_stdin_when_both_none() {
        let result = DataSource::from(None, None);
        let source = result.unwrap();

        match source {
            DataSource::Stdin => {},
            _ => panic!("Expected DataSource::Stdin"),
        }
    }

    #[test]
    fn from_returns_file_path_when_path_passed() {
        let result = DataSource::from(Some("/foo/bar"), None).unwrap();

        match result {
            DataSource::File(file_path) => assert_eq!(file_path, "/foo/bar"),
            _ => panic!("Expected DataSource::File"),
        }
    }

    #[test]
    fn from_returns_stdin_when_file_path_is_dash() {
        let result = DataSource::from(Some("-"), None).unwrap();

        match result {
            DataSource::Stdin => {},
            _ => panic!("Expected DataSource::Stdin"),
        }
    }

    #[test]
    fn from_returns_data_when_data_is_passed() {
        let result = DataSource::from(None, Some("foo")).unwrap();

        match result {
            DataSource::Data(data) => assert_eq!(data, "foo"),
            _ => panic!("Expected DataSource::Data"),
        }
    }
}
