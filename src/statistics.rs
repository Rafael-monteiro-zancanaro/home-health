enum TopicHealth {
    OK(String),
    WARNING(String, String),
    ERROR(String, String)
}

impl ToString for TopicHealth {
    fn to_string(&self) -> String {
        return match self {
            Self::OK(message) => String::from(message),
            Self::WARNING(message, hint) => format!("{} Hint: {}", message, hint),
            Self::ERROR(message, reason) => format!("{} Reason: {}", message, reason)
        }
    }
}






