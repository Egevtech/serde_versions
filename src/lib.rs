use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat(usize),
    NumberParseError(std::num::ParseIntError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(given) => write!(f, "Expected 3 parts, got {}", given),
            ParseError::NumberParseError(err) => write!(f, "Error parsing number: {}", err),
        }
    }
}

impl std::error::Error for ParseError {}

impl std::str::FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(ParseError::InvalidFormat(parts.len()));
        }

        Ok(Version {
            major: parts[0]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
            minor: parts[1]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
            patch: parts[2]
                .parse::<u32>()
                .map_err(ParseError::NumberParseError)?,
        })
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

#[macro_export]
macro_rules! ver {
    ($($version:tt)*) => {
        match <$crate::Version as std::str::FromStr>::from_str(stringify!($($version)*)) {
            Ok(v) => v,
            Err(e) => panic!("Invalid version string in macro: {}", e),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::str::FromStr;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        version: Version,
    }

    #[test]
    fn test() {
        let json_str = r#"
            {
                "version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                }
            }
            "#;

        let dt: Data = match serde_json::from_str(json_str) {
            Ok(dt) => dt,
            Err(err) => panic!("Failed to unwrap json: {}", err),
        };

        assert_eq!(
            dt,
            Data {
                version: Version::new(1, 0, 0)
            }
        )
    }

    #[test]
    fn ord_test() {
        let ver1: Version = Version::new(1, 0, 0);
        let ver2: Version = Version::new(1, 0, 1);
        let ver3: Version = Version::new(2, 3, 1);
        let ver4: Version = Version::new(0, 0, 0);

        /*
         * ver1 < ver2
         * ver2 < ver3
         * ver3 > ver4
         * ver1 > ver4
         */

        assert!(ver1 < ver2);
        assert!(ver2 < ver3);
        assert!(ver3 > ver4);
        assert!(ver1 > ver4);
    }

    #[test]
    fn eq_test() {
        let ver1: Version = Version::new(1, 0, 0);
        let ver2: Version = Version::new(1, 0, 0);
        let ver3: Version = Version::new(1, 2, 3);

        assert_eq!(ver1, ver2);
        assert_ne!(ver2, ver3);
    }

    #[test]
    fn to_string_test() {
        let ver: Version = Version::new(1, 4, 5);

        assert_eq!(ver.to_string(), "1.4.5");
    }

    #[test]
    fn from_str_test() {
        let ver: Version = Version::new(8, 7, 5);
        let str = "8.7.5";

        assert_eq!(Version::from_str(str).unwrap(), ver);
    }

    #[test]
    fn macro_test() {
        let ver: Version = Version::new(5, 4, 9);
        let macro_ver: Version = ver!(5.4.9);

        assert_eq!(ver, macro_ver);
    }
}
