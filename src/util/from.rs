use chrono::*;
use util::types::*;
use std::str::FromStr;
use util::error::ErrorKind;

impl From<f64> for InnerType {
    fn from(f: f64) -> InnerType {
        InnerType::Float(f)
    }
}


impl From<i64> for InnerType {
    fn from(i: i64) -> InnerType {
        InnerType::Int64(i)
    }
}

impl From<i32> for InnerType {
    fn from(i: i32) -> InnerType {
        InnerType::Int32(i)
    }
}

impl<'a> From<&'a i64> for InnerType {
    fn from(i: &'a i64) -> InnerType {
        InnerType::Int64(*i)
    }
}

impl<'a> From<&'a i32> for InnerType {
    fn from(i: &'a i32) -> InnerType {
        InnerType::Int32(*i)
    }
}

impl<'a, 'b> From<&'b &'a str> for InnerType {
    fn from(s: &'b &'a str) -> InnerType {
        InnerType::Str(s.to_string())
    }
}

impl<'a> From<&'a str> for InnerType {
    fn from(s: &'a str) -> InnerType {
        InnerType::Str(s.to_string())
    }
}

impl From<String> for InnerType {
    fn from(s: String) -> InnerType {
        InnerType::Str(s)
    }
}



impl<'a> From<&'a String> for InnerType {
    fn from(s: &'a String) -> InnerType {
        InnerType::Str(s.to_owned())
    }
}

impl<'a, 'b> From<&'b &'a str> for OuterType {
    fn from(s: &'b &'a str) -> OuterType {
        OuterType::Str(s.to_string())
    }
}

impl<'a> From<&'a str> for OuterType {
    fn from(s: &'a str) -> OuterType {
        OuterType::Str(s.to_string())
    }
}

impl From<String> for OuterType {
    fn from(s: String) -> OuterType {
        OuterType::Str(s)
    }
}

impl<'a> From<&'a String> for OuterType {
    fn from(s: &'a String) -> OuterType {
        OuterType::Str(s.to_owned())
    }
}

impl<'a> From<DateTime<UTC>> for OuterType {
    fn from(d: DateTime<UTC>) -> OuterType {
        OuterType::Date(d.to_owned())
    }
}

impl From<i64> for OuterType {
    fn from(i: i64) -> OuterType {
        OuterType::Int64(i)
    }
}


impl From<i32> for OuterType {
    fn from(i: i32) -> OuterType {
        OuterType::Int32(i)
    }
}


impl<'a> From<&'a i64> for OuterType {
    fn from(i: &'a i64) -> OuterType {
        OuterType::Int64(*i)
    }
}

impl<'a> From<&'a i32> for OuterType {
    fn from(i: &'a i32) -> OuterType {
        OuterType::Int32(*i)
    }
}

impl<'a> From<usize> for OuterType {
    fn from(i: usize) -> OuterType {
        OuterType::USize(i)
    }
}

impl FromStr for InnerType {
    type Err = ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<f64>() {
            return Ok(InnerType::Float(x));
        }
        if let Ok(x) = s.parse::<i64>() {
            return Ok(InnerType::Int64(x));
        }
        if let Ok(x) = s.parse::<i32>() {
            return Ok(InnerType::Int32(x));
        }
        if let Ok(x) = s.parse::<String>() {
            return Ok(InnerType::Str(x));
        }
        Err(ErrorKind::ParseError(s.into()))
    }
}