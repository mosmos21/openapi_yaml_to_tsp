use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]

pub enum ContentType {
    ApplicationJson,
    ApplicationXWwwFormUrlencoded,
    MultipartFormData,
    TextPlain,
}

impl FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "application/json" => Ok(ContentType::ApplicationJson),
            "application/x-www-form-urlencoded" => Ok(ContentType::ApplicationXWwwFormUrlencoded),
            "multipart/form-data" => Ok(ContentType::MultipartFormData),
            "text/plain" => Ok(ContentType::TextPlain),
            _ => Err(format!(
                "[RequestBodyType::from_str] invalid request body type {s}"
            )),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_type = match self {
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationXWwwFormUrlencoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::TextPlain => "text/plain",
        };
        write!(f, "{}", content_type)
    }
}
