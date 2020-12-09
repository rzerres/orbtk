/// Used to align an icon inside a stack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconAlignment {
    End,
    Start,
}

impl IconAlignment { }

impl Default for IconAlignment {
    fn default() -> Self {
        IconAlignment::Start
    }
}

impl From<&str> for IconAlignment {
    fn from(s: &str) -> Self {
        match s {
            "End" | "end" => IconAlignment::End,
            "Start" | "start" => IconAlignment::Start,
            _ => IconAlignment::default(),
        }
    }
}

impl From<String> for IconAlignment {
    fn from(s: String) -> Self {
        match s.as_str() {
            "End" | "end" => IconAlignment::End,
            "Start" | "start" => IconAlignment::Start,
            _ => IconAlignment::Start,
        }
    }
}

impl ToString for IconAlignment {
    /// Converts a valid icon enumeration to a string value
    fn to_string(&self) -> String {
        match self {
            IconAlignment::End => "end".to_string(),
            IconAlignment::Start => "start".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    // from() also implements into() for free
    fn test_into() {
        let alignment: IconAlignment = "Start".into();
        assert_eq!(alignment, IconAlignment::Start);

        let alignment: IconAlignment = "start".into();
        assert_eq!(alignment, IconAlignment::Start);

        let alignment: IconAlignment = "End".into();
        assert_eq!(alignment, IconAlignment::End);

        let alignment: IconAlignment = "end".into();
        assert_eq!(alignment, IconAlignment::End);
    }
}
