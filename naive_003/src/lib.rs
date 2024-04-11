// Challenge: How can this code be improved?

use std::str::FromStr;

pub struct FooId(String);
pub struct BarId(String);
pub struct BazId(String);

/* we need an error type */
pub enum ParseError {
    Foo,
    Bar,
    Baz,
}

/* str.parse() constructors() */

impl FromStr for FooId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // imagine some foo-specific parsing here
        Ok(Self(s.to_string()))
    }
}

impl FromStr for BarId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // imagine some bar-specific parsing here
        Ok(Self(s.to_string()))
    }
}

impl FromStr for BazId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // imagine some baz-specific parsing here
        Ok(Self(s.to_string()))
    }
}

/* Our own trait for accessing the IDs as a string */

pub trait StringIdentifier {
    fn as_str(&self) -> &str;
    fn into_string(self) -> String;
}

impl StringIdentifier for FooId {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_string(self) -> String {
        self.0
    }
}

impl StringIdentifier for BarId {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_string(self) -> String {
        self.0
    }
}

impl StringIdentifier for BazId {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_string(self) -> String {
        self.0
    }
}
