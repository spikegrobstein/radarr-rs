use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct UnableToAddMovie {
    msg: String,
}

impl UnableToAddMovie {
    pub fn with_msg(msg: &str) -> UnableToAddMovie {
        UnableToAddMovie {
            msg: String::from(msg),
        }
    }
}

impl fmt::Display for UnableToAddMovie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to add movie: {}", self.msg)
    }

}

impl error::Error for UnableToAddMovie {
    fn description(&self) -> &str {
        "unable to add movie"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct ConfigNotMaterializeable {
    missing_fields: Vec<String>,
}

impl ConfigNotMaterializeable {
    pub fn with_fields(missing_fields: Vec<String>) -> ConfigNotMaterializeable {
        ConfigNotMaterializeable {
            missing_fields,
        }
    }
}

impl fmt::Display for ConfigNotMaterializeable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration needed: {}", self.missing_fields.join("-"))
    }
}

impl error::Error for ConfigNotMaterializeable {
    fn description(&self) -> &str {
        "unable to materialize config"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
