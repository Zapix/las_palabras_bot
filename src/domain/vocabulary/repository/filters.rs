#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsVerifiedFilter {
    True,
    False,
    Any,
}

impl IsVerifiedFilter {
    pub fn as_option(&self) -> Option<bool> {
        match self {
            IsVerifiedFilter::True => Some(true),
            IsVerifiedFilter::False => Some(false),
            IsVerifiedFilter::Any => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            IsVerifiedFilter::True => "true",
            IsVerifiedFilter::False => "false",
            IsVerifiedFilter::Any => "",
        }
    }
}

impl From<&str> for IsVerifiedFilter {
    fn from(value: &str) -> Self {
        match value {
            "true" => IsVerifiedFilter::True,
            "false" => IsVerifiedFilter::False,
            _ => IsVerifiedFilter::Any,
        }
    }
}

impl From<Option<bool>> for IsVerifiedFilter {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => IsVerifiedFilter::True,
            Some(false) => IsVerifiedFilter::False,
            None => IsVerifiedFilter::Any,
        }
    }
}
